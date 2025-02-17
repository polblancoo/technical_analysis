
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Style, Modifier},
    text::{Span, Spans, Text, Line},
    widgets::{Block, Borders, Dataset, Paragraph, canvas::{Canvas, Context, Line as CanvasLine, Rectangle as CanvasRectangle}, Chart, Axis},
    Frame,
};
//mod predicion;
use crate::predicion;
use crate::predicion::*;
use crate::predicion::calculate_rsi;
use crate::predicion::calculate_macd;
use crate::predicion::calculate_bollinger_bands;
use crate::predicion::calculate_momentum;
use crate::predicion::calculate_sma;

#[derive(Debug, Clone, PartialEq)]
pub enum ChartType {
    Candlestick,
    Line,
    Dots,
    Bars,
    MACD,
    SMA,
    RSI,
    BollingerBands,
    Momentum,
}

impl ChartType {
    pub fn next(&self) -> Self {
        match self {
            ChartType::Candlestick => ChartType::Line,
            ChartType::Line => ChartType::Dots,
            ChartType::Dots => ChartType::Bars,
            ChartType::Bars => ChartType::MACD,
            ChartType::MACD => ChartType::SMA,
            ChartType::SMA => ChartType::RSI,
            ChartType::RSI => ChartType::BollingerBands,
            ChartType::BollingerBands => ChartType::Momentum,
            ChartType::Momentum => ChartType::Candlestick,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            ChartType::Candlestick => "Velas",
            ChartType::Line => "Línea",
            ChartType::Dots => "Puntos",
            ChartType::Bars => "Barras",
            ChartType::MACD => "MACD",
            ChartType::SMA => "SMA",
            ChartType::RSI => "RSI",
            ChartType::BollingerBands => "Bollinger Bands",
            ChartType::Momentum => "Momentum",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Candle {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

pub fn draw_chart<B: Backend>(
    f: &mut Frame<B>,
    data: &Vec<(String, f64)>,
    prediction_value: f64,
    chart_type: &ChartType,
) -> Result<(), Box<dyn std::error::Error>> {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(size);

    let upper_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(chunks[0]);

    let info = create_info_panel(data);
    f.render_widget(info, upper_chunks[0]);

    match chart_type {
        ChartType::Candlestick => {
            let candles = convert_to_candles(data, 24);
            draw_candlestick_view(f, &candles, upper_chunks[1])?;
        },
        ChartType::Line => {
            draw_line_view(f, data, upper_chunks[1])?;
        },
        ChartType::Dots => {
            draw_dots_view(f, data, upper_chunks[1])?;
        },
        ChartType::Bars => {
            draw_bars_view(f, data, upper_chunks[1])?;
        },
        ChartType::MACD => {
            draw_macd_view(f, data, upper_chunks[1])?;
        },
        ChartType::SMA => {
            draw_sma_view(f, data, upper_chunks[1])?;
        },
        ChartType::RSI => {
            draw_rsi_view(f, data, upper_chunks[1])?;
        },
        ChartType::BollingerBands => {
            draw_bollinger_bands_view(f, data, upper_chunks[1])?;
        },
        ChartType::Momentum => {
            draw_momentum_view(f, data, upper_chunks[1])?;
        },
    }

    let controls = create_control_panel(chart_type);
    f.render_widget(controls, chunks[1]);

    Ok(())
}

pub fn create_info_panel(data: &Vec<(String, f64)>) -> Paragraph<'static> {
    let max_value = data.iter().map(|(_, v)| v).fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min_value = data.iter().map(|(_, v)| v).fold(f64::INFINITY, |a, &b| a.min(b));
    let avg_value = data.iter().map(|(_, v)| v).sum::<f64>() / data.len() as f64;
    let last_value = data.last().map(|(_, v)| *v).unwrap_or(0.0);
// Calcular indicadores técnicos
    let rsi = predicion::calculate_rsi(data, 14);
    let (macd_line, signal_line) = predicion::calculate_macd(data);
    let (upper_band, middle_band, lower_band) = predicion::calculate_bollinger_bands(data, 20);
    let momentum = predicion::calculate_momentum(data, 14);
    let sma = predicion::calculate_sma(data, 20);
    
    let info_text = vec![
        Line::from(vec![
            Span::styled("Información del Mercado", Style::default().fg(Color::Green))
        ]),
        Line::from(vec![
            Span::styled("Máximo: ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("${:.2}", max_value), Style::default().fg(Color::White))
        ]),
        Line::from(vec![
            Span::styled("Mínimo: ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("${:.2}", min_value), Style::default().fg(Color::White))
        ]),
        Line::from(vec![
            Span::styled("Promedio: ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("${:.2}", avg_value), Style::default().fg(Color::White))
        ]),
        Line::from(vec![
            Span::styled("Último: ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("${:.2}", last_value), Style::default().fg(Color::White))
        ]),


 Line::from(vec![
            Span::styled("RSI (14): ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("{:.2}", rsi), Style::default().fg(Color::White))
        ]),
        Line::from(vec![
            Span::styled("MACD: ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("{:.2}", macd_line), Style::default().fg(Color::White)),
            Span::styled(" / Señal: ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("{:.2}", signal_line), Style::default().fg(Color::White))
        ]),
       Line::from(vec![
    Span::styled("Bollinger Bands - Superior: ", Style::default().fg(Color::Yellow)),
    Span::styled(format!("{:.2}", upper_band), Style::default().fg(Color::White))
]),
Line::from(vec![
    Span::styled("Bollinger Bands - Medio: ", Style::default().fg(Color::Yellow)),
    Span::styled(format!("{:.2}", middle_band), Style::default().fg(Color::White))
]),
Line::from(vec![
    Span::styled("Bollinger Bands - Inferior: ", Style::default().fg(Color::Yellow)),
    Span::styled(format!("{:.2}", lower_band), Style::default().fg(Color::White))
]),        Line::from(vec![
            Span::styled("Momentum (14): ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("{:.2}%", momentum), Style::default().fg(Color::White))
        ]),
        Line::from(vec![
            Span::styled("SMA (20): ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("${:.2}", sma), Style::default().fg(Color::White))
        ]),



    ];

    Paragraph::new(info_text)
        .block(Block::default().title("Estadísticas").borders(Borders::ALL))
        .alignment(Alignment::Left)
}

fn create_control_panel(current_type: &ChartType) -> Paragraph<'static> {
    let text = vec![
        Span::styled("Controles: ", Style::default().fg(Color::White)),
        Span::styled("Q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Salir | "),
        Span::styled("T", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(format!(" Cambiar vista (actual: {}) ", current_type.as_str())),
    ];

    Paragraph::new(Line::from(text))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)))
        .alignment(Alignment::Center)
}

fn draw_candlestick_view<B: Backend>(
    f: &mut Frame<B>,
    candles: &Vec<Candle>,
    area: Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    let max_price = candles.iter().map(|c| c.high).fold(f64::NEG_INFINITY, f64::max);
    let min_price = candles.iter().map(|c| c.low).fold(f64::INFINITY, f64::min);
    let price_range = max_price - min_price;
    let y_min = min_price - (price_range * 0.05);
    let y_max = max_price + (price_range * 0.05);

    let canvas = Canvas::default()
        .block(Block::default().title("Gráfico de Velas").borders(Borders::ALL))
        .x_bounds([0.0, candles.len() as f64])
        .y_bounds([y_min, y_max])
        .paint(|ctx| {
            for (i, candle) in candles.iter().enumerate() {
                let x = i as f64;
                let color = get_candle_color(candle);

                // Dibujar la mecha
                ctx.draw(&CanvasLine {
                    x1: x + 0.5,
                    y1: candle.low,
                    x2: x + 0.5,
                    y2: candle.high,
                    color,
                });

                // Dibujar el cuerpo
                let body_top = f64::max(candle.open, candle.close);
                let body_bottom = f64::min(candle.open, candle.close);
                ctx.draw(&CanvasRectangle {
                    x: x + 0.2,
                    y: body_bottom,
                    width: 0.6,
                    height: body_top - body_bottom,
                    color,
                });
            }
        });

    f.render_widget(canvas, area);
    Ok(())
}

fn get_candle_color(candle: &Candle) -> Color {
    if candle.close > candle.open {
        Color::Green
    } else {
        Color::Red
    }
}

fn convert_to_candles(data: &[(String, f64)], period: usize) -> Vec<Candle> {
    let mut candles = Vec::new();
    for chunk in data.chunks(period) {
        if !chunk.is_empty() {
            let open = chunk[0].1;
            let close = chunk[chunk.len() - 1].1;
            let high = chunk.iter().map(|(_, price)| *price).fold(f64::NEG_INFINITY, f64::max);
            let low = chunk.iter().map(|(_, price)| *price).fold(f64::INFINITY, f64::min);

            candles.push(Candle {
                date: chunk[0].0.clone(),
                open,
                high,
                low,
                close,
            });
        }
    }
    candles
}

fn create_x_axis(data: &[(String, f64)]) -> Axis<'static> {
    let data_len = data.len() as f64;
    let num_labels = 6;
    let step = (data_len / (num_labels - 1) as f64).floor() as usize;

    let mut labels = Vec::new();
    for i in 0..num_labels {
        let idx = (i * step).min(data.len() - 1);
        if idx < data.len() {
            labels.push(Span::styled(
                data[idx].0.clone(),
                Style::default().fg(Color::Gray)
            ));
        }
    }

    Axis::default()
        .title("Tiempo")
        .style(Style::default().fg(Color::Gray))
        .bounds([0.0, data_len])
        .labels(labels)
}

fn create_y_axis(data: &[(String, f64)]) -> Axis<'static> {
    let max_value = data.iter().map(|(_, v)| v).fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min_value = data.iter().map(|(_, v)| v).fold(f64::INFINITY, |a, &b| a.min(b));
    let range = max_value - min_value;
    let bounds = [
        (min_value - range * 0.05).max(0.0),
        max_value + range * 0.05
    ];

    let num_labels = 5;
    let step = (bounds[1] - bounds[0]) / (num_labels - 1) as f64;

    let labels: Vec<Span> = (0..num_labels)
        .map(|i| {
            let value = bounds[0] + step * i as f64;
            Span::styled(
                format!("${:.0}", value),
                Style::default().fg(Color::Gray)
            )
        })
        .collect();

    Axis::default()
        .title("Precio")
        .style(Style::default().fg(Color::Gray))
        .bounds(bounds)
        .labels(labels)
}

fn draw_line_view<B: Backend>(
    f: &mut Frame<B>,
    data: &[(String, f64)],
    area: Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    let line_data: Vec<(f64, f64)> = data.iter()
        .enumerate()
        .map(|(i, (_, price))| (i as f64, *price))
        .collect();

    let datasets = vec![Dataset::default()
        .name("Precio")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .data(&line_data)
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Gráfico de Línea").borders(Borders::ALL))
        .x_axis(create_x_axis(data))
        .y_axis(create_y_axis(data));

    f.render_widget(chart, area);
    Ok(())
}

fn draw_dots_view<B: Backend>(
    f: &mut Frame<B>,
    data: &[(String, f64)],
    area: Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    let dot_data: Vec<(f64, f64)> = data.iter()
        .enumerate()
        .map(|(i, (_, price))| (i as f64, *price))
        .collect();

    let datasets = vec![Dataset::default()
        .name("Precio")
        .marker(symbols::Marker::Dot)
        .style(Style::default().fg(Color::Cyan))
        .data(&dot_data)
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Gráfico de Puntos").borders(Borders::ALL))
        .x_axis(create_x_axis(data))
        .y_axis(create_y_axis(data));

    f.render_widget(chart, area);
    Ok(())
}

fn draw_bars_view<B: Backend>(
    f: &mut Frame<B>,
    data: &[(String, f64)],
    area: Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    let bar_data: Vec<(f64, f64)> = data.iter()
        .enumerate()
        .map(|(i, (_, price))| (i as f64, *price))
        .collect();

    let datasets = vec![Dataset::default()
        .name("Precio")
        .marker(symbols::Marker::Block)
        .style(Style::default().fg(Color::Cyan))
        .data(&bar_data)
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Gráfico de Barras").borders(Borders::ALL))
        .x_axis(create_x_axis(data))
        .y_axis(create_y_axis(data));

    f.render_widget(chart, area);
    Ok(())
}

fn draw_macd_view<B: Backend>(
    f: &mut Frame<B>,
    data: &[(String, f64)],
    area: Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_vec: Vec<(String, f64)> = data.to_vec();
    let (macd_line, signal_line) = calculate_macd(&data_vec);
    let macd_data: Vec<(f64, f64)> = data.iter()
        .enumerate()
        .map(|(i, _)| (i as f64, macd_line))
        .collect();
    let signal_data: Vec<(f64, f64)> = data.iter()
        .enumerate()
        .map(|(i, _)| (i as f64, signal_line))
        .collect();

    let datasets = vec![
        Dataset::default()
            .name("MACD")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(&macd_data),
        Dataset::default()
            .name("Signal")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Red))
            .data(&signal_data),
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Gráfico de MACD").borders(Borders::ALL))
        .x_axis(create_x_axis(data))
        .y_axis(create_y_axis(data));

    f.render_widget(chart, area);
    Ok(())
}

fn draw_sma_view<B: Backend>(
    f: &mut Frame<B>,
    data: &[(String, f64)],
    area: Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_vec: Vec<(String, f64)> = data.to_vec();
    let sma_value = calculate_sma(&data_vec, 20);
    let sma_data: Vec<(f64, f64)> = data.iter()
        .enumerate()
        .map(|(i, _)| (i as f64, sma_value))
        .collect();

    let datasets = vec![Dataset::default()
        .name("SMA")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .data(&sma_data)
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Gráfico de SMA").borders(Borders::ALL))
        .x_axis(create_x_axis(data))
        .y_axis(create_y_axis(data));

    f.render_widget(chart, area);
    Ok(())
}

fn draw_rsi_view<B: Backend>(
    f: &mut Frame<B>,
    data: &[(String, f64)],
    area: Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_vec: Vec<(String, f64)> = data.to_vec();
    let rsi_value = calculate_rsi(&data_vec, 14);
    let rsi_data: Vec<(f64, f64)> = data.iter()
        .enumerate()
        .map(|(i, _)| (i as f64, rsi_value))
        .collect();

    let datasets = vec![Dataset::default()
        .name("RSI")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .data(&rsi_data)
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Gráfico de RSI").borders(Borders::ALL))
        .x_axis(create_x_axis(data))
        .y_axis(create_y_axis(data));

    f.render_widget(chart, area);
    Ok(())
}

fn draw_bollinger_bands_view<B: Backend>(
    f: &mut Frame<B>,
    data: &[(String, f64)],
    area: Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_vec: Vec<(String, f64)> = data.to_vec();
    let (upper, middle, lower) = calculate_bollinger_bands(&data_vec, 20);
    let upper_data: Vec<(f64, f64)> = data.iter()
        .enumerate()
        .map(|(i, _)| (i as f64, upper))
        .collect();
    let middle_data: Vec<(f64, f64)> = data.iter()
        .enumerate()
        .map(|(i, _)| (i as f64, middle))
        .collect();
    let lower_data: Vec<(f64, f64)> = data.iter()
        .enumerate()
        .map(|(i, _)| (i as f64, lower))
        .collect();

    let datasets = vec![
        Dataset::default()
            .name("Upper Band")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(&upper_data),
        Dataset::default()
            .name("Middle Band")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Red))
            .data(&middle_data),
        Dataset::default()
            .name("Lower Band")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Green))
            .data(&lower_data),
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Gráfico de Bollinger Bands").borders(Borders::ALL))
        .x_axis(create_x_axis(data))
        .y_axis(create_y_axis(data));

    f.render_widget(chart, area);
    Ok(())
}

fn draw_momentum_view<B: Backend>(
    f: &mut Frame<B>,
    data: &[(String, f64)],
    area: Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_vec: Vec<(String, f64)> = data.to_vec();
    let momentum_value = calculate_momentum(&data_vec, 14);
    let momentum_data: Vec<(f64, f64)> = data.iter()
        .enumerate()
        .map(|(i, _)| (i as f64, momentum_value))
        .collect();

    let datasets = vec![Dataset::default()
        .name("Momentum")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .data(&momentum_data)
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Gráfico de Momentum").borders(Borders::ALL))
        .x_axis(create_x_axis(data))
        .y_axis(create_y_axis(data));

    f.render_widget(chart, area);
    Ok(())
}
