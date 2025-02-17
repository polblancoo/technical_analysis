use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Chart, Dataset, Paragraph, Axis},
    Frame,
};
// En draw.rs, importar la función
use crate::predicion::calculate_mcd;
use crate::predicion::*;
use crate::predicion::calculate_momentum;

use crate::predicion::calculate_bollinger_bands;
use crate::predicion::calculate_rsi;
use crate::predicion::calculate_sma;
use crate::predicion::calculate_macd;
//use crate::predicion::calculate_ema;

use std::sync::Arc;
use lazy_static::lazy_static;
use std::sync::Mutex;

pub fn draw_chart<B: Backend>(
    f: &mut Frame<B>,
    data: &Vec<(String, f64)>,
    prediction_value: f64,
    using_linear: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let size = f.size();
    
    // Dividir la pantalla en dos secciones: izquierda (info) y derecha (gráfico)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),  // Info panel
            Constraint::Percentage(70),  // Chart
        ])
        .split(size);

    // Panel de información
    let max_value = data.iter().map(|(_, v)| v).fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min_value = data.iter().map(|(_, v)| v).fold(f64::INFINITY, |a, &b| a.min(b));
    let avg_value = data.iter().map(|(_, v)| v).sum::<f64>() / data.len() as f64;
        let info_text = vec![
            Line::from(vec![
                Span::styled("Información del Mercado", Style::default().fg(Color::Green))
            ]),
            Line::from(vec![
                Span::styled("Valor Máximo: ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("${:.2}", max_value), Style::default().fg(Color::White))
            ]),
            Line::from(vec![
                Span::styled("Valor Mínimo: ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("${:.2}", min_value), Style::default().fg(Color::White))
            ]),
            Line::from(vec![
                Span::styled("Promedio: ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("${:.2}", avg_value), Style::default().fg(Color::White))
            ]),
            Line::from(vec![
                Span::styled("Último Valor: ", Style::default().fg(Color::Cyan)),
                Span::styled(format!("${:.2}", data.last().unwrap().1), Style::default().fg(Color::White))
            ]),
            // Y agregar al info_text (antes del último elemento):
            Line::from(vec![
                Span::styled("MCD: ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("{}", calculate_mcd(data)), Style::default().fg(Color::White))
            ]),
            Line::from(vec![
                Span::styled("RSI (14): ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("{:.2}", calculate_rsi(data, 14)), Style::default().fg(Color::White))
            ]),
            Line::from(vec![
                Span::styled("SMA(20): ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("${:.2}", calculate_sma(data, 20)), Style::default().fg(Color::White))
            ]),
           
            Line::from(vec![
                Span::styled("Momentum: ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("{:.2}%", calculate_momentum(data, 14)), Style::default().fg(Color::White))
            ]),
            Line::from(vec![
                Span::styled("SMA(20): ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("${:.2}", calculate_sma(data, 20)), Style::default().fg(Color::White))
            ]),

        ];
        
        let info_paragraph = Paragraph::new(info_text)
            .block(Block::default().title("Estadísticas").borders(Borders::ALL))
            .alignment(Alignment::Left);


    f.render_widget(info_paragraph, chunks[0]);

    // Gráfico (ahora en la sección derecha)
    let chart = Chart::new(create_datasets(data))
        .block(Block::default().title("Precio del Mercado").borders(Borders::ALL))
        .x_axis(create_x_axis())
        .y_axis(create_y_axis());

    f.render_widget(chart, chunks[1]);

    

    let method_name = if using_linear {
        Span::styled("Regresión Lineal", Style::default().fg(Color::Green))
    } else {
     Span::styled("Promedio Móvil", Style::default().fg(Color::Yellow))

    };

    let prediction_text = vec![
    Line::from(vec![
        method_name,
        Span::raw(" | Predicción para Bitcoin")
    ]),
    Line::from(vec![
        Span::raw("Día: "),
        Span::styled("98", Style::default().fg(Color::Cyan))
    ]),
    Line::from(vec![
        Span::raw("Predicción: "),
        Span::styled(
            format!("${:.2}", prediction_value),
            Style::default().fg(Color::Magenta)
        )
    ]),
    Line::from(vec![
        Span::styled(
            "Presiona 'p' para cambiar predicción",
            Style::default().fg(Color::Gray)
        )
    ])
];




   
    Ok(())
}



lazy_static! {
    static ref CHART_DATA: Mutex<Vec<(f64, f64)>> = Mutex::new(Vec::new());
}

fn create_datasets(data: &Vec<(String, f64)>) -> Vec<Dataset<'static>> {
    let points: Vec<(f64, f64)> = data.iter()
        .enumerate()
        .map(|(i, (_, y))| (i as f64, *y))
        .collect();

    // Convertir a un vector estático
    let static_points: &'static [(f64, f64)] = Box::leak(points.into_boxed_slice());

    vec![Dataset::default()
        .name("Precio BTC")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .data(static_points)]
}
fn create_x_axis() -> Axis<'static> {
    Axis::default()
        .title("Tiempo")
        .style(Style::default().fg(Color::Gray))
        .bounds([0.0, 100.0])
        .labels(vec![
            Span::raw("Inicio"),
            Span::raw("Medio"),
            Span::raw("Fin")
        ])
}

fn create_y_axis() -> Axis<'static> {
    let bounds = [0.0, 120000.0];
    Axis::default()
        .title("Precio")
        .style(Style::default().fg(Color::Gray))
        .bounds(bounds)
        .labels(vec![
            Span::raw(format!("${:.0}", bounds[0])),
            Span::raw(format!("${:.0}", bounds[1]/2.0)),
            Span::raw(format!("${:.0}", bounds[1]))
        ])
}