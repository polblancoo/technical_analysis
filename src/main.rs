use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::io;

mod draw;
mod predicion;
mod serde_data;
use crate::draw::draw_chart;
use crate::predicion::{predict_price, predict_price_moving_average};
use crate::serde_data::load_data_from_csv;

fn main() -> Result<(), Box<dyn Error>> {
    // Cargar los datos desde el CSV
    let data = load_data_from_csv("data.csv")?;
    
    let past_days: Vec<f64> = data.iter().map(|d| d.apertura * 1000.0).collect();
    let future_data: Vec<f64> = data.iter().map(|d| d.ultimo * 1000.0).collect();

    if past_days.is_empty() || future_data.is_empty() {
        eprintln!("Error: No hay datos disponibles en el CSV.");
        return Ok(());
    }

    // Realizar la predicción inicial
    let mut current_prediction = predict_price(98.0, past_days.clone(), future_data.clone(), true)?;

    let mut using_linear = true;

   // Crear datos para el gráfico con fechas
   let mut chart_data: Vec<(String, f64)> = data.iter()
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
        terminal.draw(|f| {
            if let Err(e) = draw_chart(f, &chart_data, current_prediction, using_linear) {
                eprintln!("Error dibujando el gráfico: {}", e);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('p') => {
                        // Cambiar entre predicciones
                        using_linear = !using_linear;
                        if using_linear {
                            current_prediction = predict_price(98.0, past_days.clone(), future_data.clone(), using_linear)?;
                        } else {
                            current_prediction = predict_price_moving_average(98.0, past_days.clone(), future_data.clone())?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Limpiar terminal
    terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;

    Ok(())
}