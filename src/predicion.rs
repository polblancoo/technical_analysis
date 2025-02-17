use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};
use serde::Deserialize;
use std::borrow::Cow;
use std::error::Error;


pub fn load_data_from_csv(file_path: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let data = crate::serde_data::load_data_from_csv(file_path)?;
    let mut x = Vec::new();
    let mut y = Vec::new();

    for record in data {
        x.push(record.apertura);
        y.push(record.ultimo);
    }

    Ok((x, y))
}
pub fn linear_regression(
    future_day: f64,
    past_days: Vec<f64>,
    future_data: Vec<f64>,
) -> Result<f64, Box<dyn Error>> {
    if past_days.len() != future_data.len() {
        return Err("Los vectores x e y deben tener la misma longitud".into());
    }

    // Usar los últimos 30 días para la predicción
    let window_size = 30.min(past_days.len());
    let past_days = past_days.iter().rev().take(window_size).cloned().collect::<Vec<_>>();
    let future_data = future_data.iter().rev().take(window_size).cloned().collect::<Vec<_>>();

    // Normalizar los días para que empiecen desde 0
    let normalized_days: Vec<f64> = (0..window_size).map(|i| i as f64).collect();

    let n = window_size as f64;
    let sum_x: f64 = normalized_days.iter().sum();
    let sum_y: f64 = future_data.iter().sum();
    let sum_xy: f64 = normalized_days.iter().zip(future_data.iter()).map(|(x, y)| x * y).sum();
    let sum_xx: f64 = normalized_days.iter().map(|x| x * x).sum();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / n;

    // Predecir usando el último valor conocido como base
    let last_known_price = future_data[0];
    let days_into_future = 1.0; // Predecir solo un día hacia el futuro
    
    Ok(last_known_price + (slope * days_into_future))
}
pub fn predict_price(
    future_day: f64,
    past_days: Vec<f64>,
    future_data: Vec<f64>,
    using_linear: bool,
) -> Result<f64, Box<dyn Error>> {
    if using_linear {
        linear_regression(future_day, past_days, future_data)
    } else {
        predict_price_moving_average(future_day, past_days, future_data)
    }
}
pub fn predict_price_moving_average(
    future_day: f64,
    past_days: Vec<f64>,
    future_data: Vec<f64>,
) -> Result<f64, Box<dyn Error>> {
    if past_days.len() != future_data.len() {
        return Err("Los vectores de datos deben tener la misma longitud".into());
    }

    // Calcular el promedio móvil de los últimos 7 días
    let window_size = 7;
    let last_values = &future_data[future_data.len().saturating_sub(window_size)..];
    let prediction = last_values.iter().sum::<f64>() / last_values.len() as f64;

    Ok(prediction)
}

// En predicion.rs
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

pub fn calculate_mcd(data: &Vec<(String, f64)>) -> i64 {
    let prices: Vec<i64> = data.iter()
        .map(|(_, v)| *v as i64)  // Convertimos directamente a i64 sin multiplicar por 100
        .collect();

    if prices.len() > 1 {
        prices.windows(2)
            .map(|w| gcd(w[0], w[1]))
            .fold(prices[0], |acc, x| gcd(acc, x))
    } else {
        prices[0]
    }
}

pub fn calculate_rsi(data: &Vec<(String, f64)>, period: usize) -> f64 {
    if data.len() < period + 1 {
        return 50.0; // Valor neutral si no hay suficientes datos
    }

    let mut gains = Vec::new();
    let mut losses = Vec::new();

    // Calcular ganancias y pérdidas
    for i in 1..data.len() {
        let diff = data[i].1 - data[i-1].1;
        if diff >= 0.0 {
            gains.push(diff);
            losses.push(0.0);
        } else {
            gains.push(0.0);
            losses.push(diff.abs());
        }
    }

    // Calcular promedios
    let avg_gain: f64 = gains.iter().take(period).sum::<f64>() / period as f64;
    let avg_loss: f64 = losses.iter().take(period).sum::<f64>() / period as f64;

    if avg_loss == 0.0 {
        return 100.0;
    }

    // Calcular RS y RSI
    let rs = avg_gain / avg_loss;
    let rsi = 100.0 - (100.0 / (1.0 + rs));

    rsi
}
pub fn calculate_sma(data: &Vec<(String, f64)>, period: usize) -> f64 {
    if data.len() < period {
        return data.last().unwrap().1;
    }
    let values: Vec<f64> = data.iter().map(|(_, v)| *v).collect();
    values.iter().rev().take(period).sum::<f64>() / period as f64
}
pub fn calculate_macd(data: &Vec<(String, f64)>) -> (f64, f64) {
    let ema12 = calculate_ema(data, 12);
    let ema26 = calculate_ema(data, 26);
    let macd_line = ema12 - ema26;
    
    // Calculamos el signal line usando el mismo EMA pero con los valores del MACD
    let values_for_signal: Vec<(String, f64)> = vec![("".to_string(), macd_line)];
    let signal_line = calculate_ema(&values_for_signal, 9);
    
    (macd_line, signal_line)
}

fn calculate_ema(data: &Vec<(String, f64)>, period: usize) -> f64 {
    let multiplier = 2.0 / (period + 1) as f64;
    let values: Vec<f64> = data.iter().map(|(_, v)| *v).collect();
    let mut ema = values[0];
    
    for value in values.iter().skip(1) {
        ema = (value - ema) * multiplier + ema;
    }
    ema
}
pub fn calculate_bollinger_bands(data: &Vec<(String, f64)>, period: usize) -> (f64, f64, f64) {
    let sma = calculate_sma(data, period);
    let values: Vec<f64> = data.iter().map(|(_, v)| *v).collect();
    let variance: f64 = values.iter()
        .rev()
        .take(period)
        .map(|x| (x - sma).powi(2))
        .sum::<f64>() / period as f64;
    let std_dev = variance.sqrt();
    
    (sma + (2.0 * std_dev), sma, sma - (2.0 * std_dev))
}
pub fn calculate_momentum(data: &Vec<(String, f64)>, period: usize) -> f64 {
    if data.len() < period {
        return 0.0;
    }
    let current_price = data.last().unwrap().1;
    let past_price = data[data.len() - period - 1].1;
    ((current_price - past_price) / past_price) * 100.0
}