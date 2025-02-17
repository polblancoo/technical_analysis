use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::io;
use std::path::Path;
use std::env;

mod draw;
mod predicion;
mod serde_data;
use crate::draw::{draw_chart, ChartType};
use crate::predicion::{predict_price, predict_price_moving_average};
use crate::serde_data::load_data_from_csv;

fn main() -> Result<(), Box<dyn Error>> {
    // Imprimir el directorio de trabajo actual
    let cwd = env::current_dir()?;
    println!("Directorio de trabajo actual: {:?}", cwd);

    // Cargar los datos desde el CSV
    let file_path = "data.csv"; // Asegúrate de que esta ruta sea correcta
    if !Path::new(file_path).exists() {
        eprintln!("Error: El archivo {} no existe.", file_path);
        return Ok(());
    }

    let data = load_data_from_csv(file_path)?;

    let past_days: Vec<f64> = data.iter().map(|d| d.apertura * 1000.0).collect();
    let future_data: Vec<f64> = data.iter().map(|d| d.ultimo * 1000.0).collect();

    if past_days.is_empty() || future_data.is_empty() {
        eprintln!("Error: No hay datos disponibles en el CSV.");
        return Ok(());
    }

    // Realizar la predicción inicial
    let mut current_prediction = predict_price(98.0, past_days.clone(), future_data.clone(), true)?;

    let mut using_linear = true;
    let mut chart_type = ChartType::Line;

    // Crear datos para el gráfico con fechas
    let mut chart_data: Vec<(String, f64)> = data
        .iter()
        .map(|d| (d.fecha.clone(), d.ultimo * 1000.0))
        .collect();
    // Ordenar los datos por fecha de más antigua a más reciente
    chart_data.reverse();

    // Configurar terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Bucle principal que espera 'q' o Esc para salir
    loop {
        let info = draw::create_info_panel(&chart_data);
        terminal.draw(|f| {
            f.render_widget(info.clone(), f.size());
            if let Err(e) = draw_chart(f, &chart_data, current_prediction, &chart_type) {
                eprintln!("Error dibujando el gráfico: {}", e);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('t') => {
                        // Cambiar entre predicciones
                        chart_type = chart_type.next();
                        match chart_type {
                            ChartType::Line => {
                                current_prediction = predict_price(
                                    98.0,
                                    past_days.clone(),
                                    future_data.clone(),
                                    true,
                                )?;
                            },
                            ChartType::Candlestick => {
                                current_prediction = predict_price(
                                    98.0,
                                    past_days.clone(),
                                    future_data.clone(),
                                    false,
                                )?;
                            },
                            ChartType::MACD => {
                                let (macd_line, _) = predicion::calculate_macd(&chart_data);
                                current_prediction = macd_line;
                            },
                            ChartType::SMA => {
                                current_prediction = predicion::calculate_sma(&chart_data, 20);
                            },
                            ChartType::RSI => {
                                current_prediction = predicion::calculate_rsi(&chart_data, 14);
                            },
                            ChartType::BollingerBands => {
                                let (upper, middle, _) = predicion::calculate_bollinger_bands(&chart_data, 20);
                                current_prediction = middle;
                            },
                            ChartType::Momentum => {
                                current_prediction = predicion::calculate_momentum(&chart_data, 14);
                            },
                            _ => {},
                        }
                        // Forzar un redibujo del gráfico
                        terminal.draw(|f| {
                            f.render_widget(info.clone(), f.size());
                            if let Err(e) = draw_chart(f, &chart_data, current_prediction, &chart_type) {
                                eprintln!("Error dibujando el gráfico: {}", e);
                            }
                        })?;
                    }
                    _ => {}
                }
            }
        }
    }

    // Limpiar terminal
    terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
