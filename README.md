# Proyecto de Visualización y Predicción de Datos Financieros

Este proyecto es una herramienta de visualización y predicción de datos financieros, diseñada para analizar y predecir tendencias en los precios de activos financieros. Utiliza Rust junto con la biblioteca `ratatui` para crear una interfaz de terminal interactiva que muestra gráficos y estadísticas en tiempo real.

## Características Principales

- **Visualización de Gráficos**: Soporta múltiples tipos de gráficos, incluyendo:
  - **Velas (Candlestick)**
  - **Línea**
  - **Puntos**
  - **Barras**
  - **MACD**
  - **SMA (Media Móvil Simple)**
  - **RSI (Índice de Fuerza Relativa)**
  - **Bandas de Bollinger**
  - **Momentum**

- **Indicadores Técnicos**: Calcula y muestra indicadores técnicos como:
  - **RSI**
  - **MACD**
  - **Bandas de Bollinger**
  - **Momentum**
  - **SMA**

- **Predicción de Precios**: Incluye algoritmos para predecir precios futuros basados en:
  - **Regresión Lineal**
  - **Media Móvil**

- **Interfaz de Usuario Interactiva**: Permite al usuario cambiar entre diferentes tipos de gráficos y ver estadísticas en tiempo real.

## Requisitos

- **Rust**: Asegúrate de tener Rust instalado en tu sistema. Puedes instalarlo desde [rustup.rs](https://rustup.rs/).

## Instalación

1. Clona el repositorio:
   ```bash
   git clone https://github.com/tu-usuario/tu-repositorio.git
   cd tu-repositorio

Compila el proyecto:

bash
Copy
cargo build --release
Ejecuta el programa:

bash
Copy
cargo run --release
Uso
Cambiar entre gráficos: Presiona la tecla T para cambiar entre los diferentes tipos de gráficos disponibles.

Salir: Presiona Q o Esc para salir del programa.

Estructura del Proyecto
main.rs: Punto de entrada del programa. Configura la terminal y maneja el bucle principal de la aplicación.

draw.rs: Contiene la lógica para dibujar los gráficos y el panel de información en la terminal.

predicion.rs: Implementa los algoritmos de predicción y cálculo de indicadores técnicos.

serde_data.rs: Maneja la carga y deserialización de datos desde un archivo CSV.

Ejemplo de Datos
El programa espera un archivo CSV con los siguientes campos:

fecha: Fecha en formato YYYY-MM-DD.

ultimo: Precio de cierre.

apertura: Precio de apertura.

maximo: Precio máximo del día.

minimo: Precio mínimo del día.

vol.: Volumen de operaciones.

% var.: Variación porcentual.

Contribuciones
¡Las contribuciones son bienvenidas! Si deseas mejorar el proyecto, por favor abre un issue o envía un pull request.

Licencia
Este proyecto está bajo la licencia MIT. Consulta el archivo LICENSE para más detalles.

¡Gracias por usar esta herramienta! Si tienes alguna pregunta o sugerencia, no dudes en contactarme.

Copy


### Explicación del README:

1. **Introducción**: Breve descripción del proyecto y su propósito.
2. **Características Principales**: Lista de las funcionalidades clave del proyecto.
3. **Requisitos**: Indica que se necesita Rust para ejecutar el proyecto.
4. **Instalación**: Pasos para clonar, compilar y ejecutar el proyecto.
5. **Uso**: Explica cómo interactuar con la aplicación.
6. **Estructura del Proyecto**: Describe los archivos principales y su función.
7. **Ejemplo de Datos**: Especifica el formato esperado del archivo CSV.
8. **Contribuciones**: Invita a contribuir al proyecto.
9. **Licencia**: Indica la licencia bajo la cual se distribuye el proyecto.

