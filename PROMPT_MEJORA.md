# Prompt para Mejorar el Codigo Base

Copia y pega el contenido del bloque de abajo en un asistente de IA (Claude, ChatGPT)
para obtener un ZIP con el proyecto completo y arrancable.

Si preferis trabajar en tu editor con un agente local (Claude Code, Cursor, Copilot), usa `AGENTS.md` en vez de este archivo: dice lo mismo pero para que escriba los archivos en disco.

## Las dos reglas que no se negocian

1. **Completa el boilerplate.** Todo lo que el proyecto necesita para compilar y arrancar: manifiesto de dependencias, punto de entrada, configuracion, capa de interfaz, y las capas del patron arquitectonico declarado. Eso es andamiaje y es tu trabajo.
2. **NO resuelvas el reto.** Los entregables de las fases son el trabajo de la persona. El hueco pedagogico se deja como esta: el proyecto arranca, pero lo que el reto pide implementar NO esta implementado.

Dicho de otra forma: si algo impide compilar, arreglalo. Si algo es logica de negocio incompleta, validaciones ausentes, un secreto hardcodeado o un patron mejorable, dejalo exactamente como esta — es lo que la persona tiene que encontrar.

## Lo que le falta a este proyecto

Esto NO lo tenes que adivinar: salio de comparar el proyecto contra la arquitectura declarada del reto y de un analisis estatico del codigo. Completalo TODO.

### Boilerplate del stack que falta

Sin esto no compila ni arranca. Es andamiaje, no toca nada de lo pedagogico:

- **Punto de entrada del stack elegido** — Sin un punto de entrada reconocible, el runtime no tiene por donde arrancar la aplicacion.
- **Capa de interfaz (controller/handler)** — Sin una capa de interfaz explicita, no hay forma de invocar la logica de negocio desde afuera del proceso.

## Como saber que terminaste

```bash
el comando de build o arranque canonico del stack elegido
```

Ese comando corriendo sin errores es la definicion de "listo".

---

```
## Briefing del reto (autoridad)
Este bloque manda sobre los archivos adjuntos. El stack y el rol salen de AQUÍ, no de un topic genérico ni de markdown placeholder.

### Contexto técnico original
Build a REST API with Rust, Actix Web and Diesel ORM

### Reto
- Tema: rust-actix-web
- Seniority: junior-l2
- Tipo: practical
- Título: Desarrollo de API REST con Rust y Actix Web
- Tiempo estimado: 8 horas

### Fases (trabajo del HUMANO — PROHIBIDO completarlas)
No implementes estos entregables. Dejalos como hueco pedagógico. El asistente solo materializa el proyecto arrancable para que el participante pueda trabajar.
- Fase 1: Definición del modelo de datos — objetivo: Definir el modelo de datos para los préstamos, incluyendo sus atributos y validaciones. — entregable (NO resolver): Modelo de datos para préstamos con atributos y validaciones definidas.
- Fase 2: Implementación de endpoints — objetivo: Implementar los endpoints para crear, leer, actualizar y eliminar préstamos. — entregable (NO resolver): Endpoints implementados para crear, leer, actualizar y eliminar préstamos, con validaciones y manejo de errores.
- Fase 3: Pruebas y validación — objetivo: Realizar pruebas y validación de la API implementada. — entregable (NO resolver): Casos de prueba ejecutados y documentados, con resultados y problemas encontrados.

Eres un asistente experto en análisis, corrección y generación de archivos de cualquier tipo:
código fuente, documentación, hojas de cálculo, documentos Word, configuraciones, entre otros.
Voy a enviarte una cadena de texto que contiene uno o más archivos. Cada archivo está delimitado por un marcador con el siguiente formato:
// === ARCHIVO: ruta/del/archivo.extension ===
o también puede aparecer como:
## === ARCHIVO: ruta/del/archivo.extension ===
Lo que sigue al marcador puede ser:

El contenido real del archivo (código, texto, YAML, etc.)
Una descripción en lenguaje natural de lo que debe contener el archivo


TU TAREA
PASO 0 — ¿Esto es un proyecto o una carcasa?
Antes de extraer archivos, leé el Briefing (si está) y diagnosticá el adjunto.

Es CARCASA si ocurre CUALQUIERA de estas:
- No hay manifiesto de dependencias del stack del briefing (manifest.json de VTEX IO / package.json / pom.xml / build.gradle / requirements.txt / go.mod / *.tf / *.csproj, según corresponda)
- Hay un "binario" que en realidad es un comentario ("no puede ser mostrado como texto plano", placeholder .fig/.docx vacío)
- Los markdowns ya completan entregables de fases posteriores ("se implementó fade-in", lista de áreas ya resuelta)

Si es CARCASA:
- MATERIALIZÁ un proyecto que arranca en el stack del briefing (VTEX IO Store Framework, Angular, Terraform, pytest, Nest, etc.). Incluí manifiesto, punto de entrada y capa de interfaz reales.
- NO copies los markdowns de "solución" como si fueran el producto. Son ruido de generación.
- NO resuelvas las fases del briefing (están marcadas PROHIBIDO). Dejá el hueco pedagógico: el flujo existe, las microinteracciones/calidad/infra que el reto pide NO están hechas.
- Después seguí al PASO 5 (ZIP).

Si es un proyecto REAL (manifiesto + código que compila o arranca):
- Seguí PASO 1 en adelante. 🔴 compilación sí. 🟡 pedagógico no.

PASO 1 — Detección y extracción
Identifica todos los archivos presentes en la cadena. Para cada archivo extrae:

Su ruta completa (ej: src/main/java/com/pragma/Service.java)
Su contenido o descripción

PASO 2 — Clasificación por tipo
Clasifica cada archivo en una de estas categorías:
A) Código fuente (Java, Python, TypeScript, JavaScript, Kotlin, etc.)
B) Configuración / documentación (YAML, properties, Markdown, JSON, txt, etc.)
C) Excel (.xlsx, .xls, .csv)
D) Word (.docx, .doc)
E) Otro tipo de archivo binario o especial
PASO 3 — Clasificación de errores en código fuente

Objetivo prioritario: que el proyecto compile. No corrijas flujo de negocio ni lógica funcional.

Antes de modificar cualquier archivo de código fuente, clasifica cada problema encontrado en una de estas dos categorías:
🔴 ERROR DE COMPILACIÓN — corregir siempre
Son errores que impiden que el proyecto arranque, sin valor pedagógico:

Import faltante o incorrecto
Clase, método o variable referenciada que no existe en ningún archivo del proyecto
Error de sintaxis
Anotación con atributos inválidos
Dependencia ausente en pom.xml, package.json, etc.
Archivo referenciado que no existe y debe ser creado con implementación mínima

→ CORREGIR estos errores.
🟡 PROBLEMA FUNCIONAL O DE CALIDAD — preservar siempre
Son problemas que no impiden compilar. Pueden ser intencionales para el aprendizaje:

Clave secreta hardcodeada ("secret", "password123")
API deprecada que funciona pero tiene reemplazo moderno
Lógica de negocio incorrecta o incompleta
Código redundante o de baja legibilidad
Falta de validaciones en flujo de negocio
Patrones de diseño incorrectos pero funcionales
Concurrencia no segura
Configuración funcional pero no óptima

→ PRESERVAR tal cual. No corregir, no mejorar, no comentar.
PASO 4 — Procesamiento según tipo de archivo
Tipo A — Código fuente
Aplica únicamente las correcciones clasificadas como 🔴 ERROR DE COMPILACIÓN.
No alteres ningún elemento clasificado como 🟡 PROBLEMA FUNCIONAL O DE CALIDAD.
Si falta un archivo referenciado, créalo con la implementación mínima necesaria para compilar.
Tipo B — Configuración / documentación
Extrae el contenido tal cual, sin modificaciones salvo errores evidentes de sintaxis
(ej: YAML mal indentado).
Tipo C — Excel (.xlsx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un archivo Excel funcional con:

Fila de encabezados en negrita con color de fondo distintivo
Columnas con ancho ajustado al contenido
Tipos de dato correctos por columna
Validaciones si la descripción lo indica
Hojas nombradas descriptivamente si hay más de una
Filas de ejemplo si no hay datos reales

Tipo D — Word (.docx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un documento Word funcional con:

Estilos de título (Título 1, Título 2) para jerarquía de secciones
Fuente legible (Calibri o equivalente), tamaño 11-12pt para cuerpo
Márgenes estándar
Tabla de contenido si tiene múltiples secciones
Tablas con encabezados en negrita si aplica

Tipo E — Otro
Genera el archivo con el contenido o estructura más apropiada según la descripción.
PASO 5 — Exportación en ZIP
Empaqueta todos los archivos en un único archivo ZIP descargable respetando exactamente
la estructura de rutas indicada por los marcadores.
El ZIP debe incluir:

Archivos de código con únicamente los errores de compilación corregidos
Archivos de configuración y documentación sin cambios
Archivos nuevos creados para resolver dependencias de compilación faltantes
Archivos Excel y Word generados desde descripción

IMPORTANTE: El ZIP debe estar listo para descargar al finalizar. No preguntes si el usuario
quiere generarlo. Simplemente genera el archivo y proporciona el enlace de descarga; No debes desplegar en el chat el resumen de lo que arreglaste al Zip, solo entregalo.

REGLAS IMPORTANTES

No omitas ningún archivo aunque no tenga errores ni modificaciones
Respeta los nombres y rutas exactas indicadas por los marcadores
Si un archivo no tiene marcador claro, infiere el nombre desde su contenido
Si la cadena contiene solo documentación, placeholders o binarios fake, NO la reproduzcas:
aplicá PASO 0 (materializar el proyecto del briefing). Reproducir la carcasa es un fallo.
No agregues texto después del enlace de descarga del ZIP
No preguntes si el usuario quiere el ZIP: simplemente generalo siempre
Si detectas que falta un archivo de configuración necesario para compilar
(pom.xml, package.json, requirements.txt, build.gradle, etc.), créalo e inclúyelo
inferiendo su contenido desde los imports y frameworks detectados en el código
Nunca corrijas problemas 🟡 aunque parezcan obvios o fáciles de mejorar.
El participante que recibirá este proyecto los debe encontrar y resolver él mismo.


INPUT
Aquí está la cadena con los archivos:

// === ARCHIVO: package.json ===
{
  "name": "rust-actix-loan-api",
  "version": "1.0.0",
  "description": "REST API para gestión de préstamos con Rust y Actix Web",
  "private": true,
  "scripts": {
    "build": "cargo build --release",
    "dev": "cargo run",
    "test": "cargo test",
    "check": "cargo check"
  },
  "keywords": [
    "rust",
    "actix-web",
    "rest-api",
    "fintech",
    "loan-management"
  ],
  "author": "",
  "license": "MIT",
  "devDependencies": {},
  "dependencies": {},
  "engines": {
    "node": ">=18.0.0"
  },
  "notes": "Este proyecto utiliza Cargo como gestor de paquetes de Rust. El archivo Cargo.toml contiene las dependencias reales del proyecto. Este package.json existe para compatibilidad con herramientas npm que puedan requerirse en el entorno de desarrollo."
}

// === ARCHIVO: src/models/loan.rs ===
use chrono::{NaiveDate, Utc};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::fmt;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoanStatus {
    Pendiente,
    Aprobado,
    Rechazado,
}

impl fmt::Display for LoanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoanStatus::Pendiente => write!(f, "pendiente"),
            LoanStatus::Aprobado => write!(f, "aprobado"),
            LoanStatus::Rechazado => write!(f, "rechazado"),
        }
    }
}

impl std::convert::From<String> for LoanStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "aprobado" => LoanStatus::Aprobado,
            "rechazado" => LoanStatus::Rechazado,
            _ => LoanStatus::Pendiente,
        }
    }
}

impl std::convert::From<&String> for LoanStatus {
    fn from(s: &String) -> Self {
        LoanStatus::from(s.clone())
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Insertable,
    AsChangeset,
    Validate,
)]
#[diesel(table_name = loans)]
#[validate(schema(
    validate(loan.amount > 0, "amount must be positive"),
    validate(loan.interest_rate > 0.0, "interest_rate must be positive"),
    validate(loan.due_date > *&Utc::now().date_naive(), "due_date must be in the future")
))]
pub struct Loan {
    pub id: i64,
    #[validate(range(min = 0.01, message = "amount must be greater than zero")]
    pub amount: f64,
    #[validate(range(min = 0.001, message = "interest_rate must be greater than zero")]
    pub interest_rate: f64,
    pub due_date: NaiveDate,
    pub status: String,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

impl Loan {
    pub fn new(
        id: i64,
        amount: f64,
        interest_rate: f64,
        due_date: NaiveDate,
    ) -> Result<Self, String> {
        if amount <= 0.0 {
            return Err("El monto del préstamo debe ser mayor a cero".to_string());
        }
        if interest_rate <= 0.0 {
            return Err("La tasa de interés debe ser mayor a cero".to_string());
        }
        let today = Utc::now().date_naive();
        if due_date <= today {
            return Err("La fecha de vencimiento debe ser posterior a la fecha actual".to_string());
        }

        let now = Utc::now().date_naive();
        Ok(Loan {
            id,
            amount,
            interest_rate,
            due_date,
            status: LoanStatus::Pendiente.to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn approve(&mut self) {
        self.status = LoanStatus::Aprobado.to_string();
        self.updated_at = Utc::now().date_naive();
    }

    pub fn reject(&mut self) {
        self.status = LoanStatus::Rechazado.to_string();
        self.updated_at = Utc::now().date_naive();
    }

    pub fn is_pending(&self) -> bool {
        self.status == LoanStatus::Pendiente.to_string()
    }

    pub fn is_approved(&self) -> bool {
        self.status == LoanStatus::Aprobado.to_string()
    }

    pub fn is_rejected(&self) -> bool {
        self.status == LoanStatus::Rechazado.to_string()
    }

    pub fn calculate_total_amount(&self) -> f64 {
        self.amount * (1.0 + self.interest_rate)
    }

    pub fn days_until_due(&self) -> i64 {
        let today = Utc::now().date_naive();
        (self.due_date - today).num_days()
    }

    pub fn is_overdue(&self) -> bool {
        self.days_until_due() < 0
    }

    pub fn update_amount(&mut self, new_amount: f64) -> Result<(), String> {
        if new_amount <= 0.0 {
            return Err("El nuevo monto debe ser mayor a cero".to_string());
        }
        self.amount = new_amount;
        self.updated_at = Utc::now().date_naive();
        Ok(())
    }

    pub fn update_interest_rate(&mut self, new_rate: f64) -> Result<(), String> {
        if new_rate <= 0.0 {
            return Err("La nueva tasa de interés debe ser mayor a cero".to_string());
        }
        self.interest_rate = new_rate;
        self.updated_at = Utc::now().date_naive();
        Ok(())
    }

    pub fn update_due_date(&mut self, new_due_date: NaiveDate) -> Result<(), String> {
        let today = Utc::now().date_naive();
        if new_due_date <= today {
            return Err("La nueva fecha de vencimiento debe ser posterior a la fecha actual".to_string());
        }
        self.due_date = new_due_date;
        self.updated_at = Utc::now().date_naive();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_loan_creation_with_valid_data() {
        let future_date = Utc::now().date_naive() + chrono::Duration::days(30);
        let loan = Loan::new(1, 1000.0, 0.1, future_date);
        assert!(loan.is_ok());
    }

    #[test]
    fn test_loan_creation_with_negative_amount() {
        let future_date = Utc::now().date_naive() + chrono::Duration::days(30);
        let loan = Loan::new(1, -100.0, 0.1, future_date);
        assert!(loan.is_err());
    }

    #[test]
    fn test_loan_creation_with_zero_interest_rate() {
        let future_date = Utc::now().date_naive() + chrono::Duration::days(30);
        let loan = Loan::new(1, 1000.0, 0.0, future_date);
        assert!(loan.is_err());
    }

    #[test]
    fn test_loan_creation_with_past_due_date() {
        let past_date = Utc::now().date_naive() - chrono::Duration::days(1);
        let loan = Loan::new(1, 1000.0, 0.1, past_date);
        assert!(loan.is_err());
    }

    #[test]
    fn test_loan_approve() {
        let future_date = Utc::now().date_naive() + chrono::Duration::days(30);
        let mut loan = Loan::new(1, 1000.0, 0.1, future_date).unwrap();
        loan.approve();
        assert!(loan.is_approved());
    }

    #[test]
    fn test_loan_reject() {
        let future_date = Utc::now().date_naive() + chrono::Duration::days(30);
        let mut loan = Loan::new(1, 1000.0, 0.1, future_date).unwrap();
        loan.reject();
        assert!(loan.is_rejected());
    }

    #[test]
    fn test_calculate_total_amount() {
        let future_date = Utc::now().date_naive() + chrono::Duration::days(30);
        let loan = Loan::new(1, 1000.0, 0.1, future_date).unwrap();
        assert_eq!(loan.calculate_total_amount(), 1100.0);
    }

    #[test]
    fn test_days_until_due() {
        let future_date = Utc::now().date_naive() + chrono::Duration::days(15);
        let loan = Loan::new(1, 1000.0, 0.1, future_date).unwrap();
        assert_eq!(loan.days_until_due(), 15);
    }

    #[test]
    fn test_is_overdue() {
        let past_date = Utc::now().date_naive() - chrono::Duration::days(1);
        let loan = Loan::new(1, 1000.0, 0.1, past_date).unwrap();
        assert!(loan.is_overdue());
    }
}

// === ARCHIVO: src/dtos/loan_dto.rs ===
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateLoanDto {
    #[validate(range(min = 0.01, message = "El monto debe ser mayor a cero")]
    pub amount: f64,
    #[validate(range(min = 0.001, message = "La tasa de interés debe ser mayor a cero")]
    pub interest_rate: f64,
    #[validate(custom = "validate_due_date")]
    pub due_date: String,
}

fn validate_due_date(date: &str) -> Result<(), validator::ValidationError> {
    match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(naive_date) => {
            let today = chrono::Utc::now().date_naive();
            if naive_date <= today {
                let mut err = validator::ValidationError::new("due_date_must_be_future");
                err.add_param(serde_json::json!({"value": date}), &"due_date");
                err.message = Some("La fecha de vencimiento debe ser posterior a la fecha actual".into());
                Err(err)
            } else {
                Ok(())
            }
        }
        Err(_) => {
            let mut err = validator::ValidationError::new("invalid_date_format");
            err.add_param(serde_json::json!({"value": date}), &"due_date");
            err.message = Some("El formato de fecha debe ser YYYY-MM-DD".into());
            Err(err)
        }
    }
}

impl CreateLoanDto {
    pub fn parse_due_date(&self) -> Result<NaiveDate, String> {
        NaiveDate::parse_from_str(&self.due_date, "%Y-%m-%d")
            .map_err(|_| "Formato de fecha inválido. Use YYYY-MM-DD".to_string())
    }

    pub fn validate_all(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.amount <= 0.0 {
            errors.push("El monto debe ser mayor a cero".to_string());
        }

        if self.interest_rate <= 0.0 {
            errors.push("La tasa de interés debe ser mayor a cero".to_string());
        }

        if let Err(e) = self.parse_due_date() {
            errors.push(e);
        } else {
            let parsed_date = self.parse_due_date().unwrap();
            let today = chrono::Utc::now().date_naive();
            if parsed_date <= today {
                errors.push("La fecha de vencimiento debe ser posterior a la fecha actual".to_string());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateLoanDto {
    #[validate(range(min = 0.01, message = "El monto debe ser mayor a cero"))]
    pub amount: Option<f64>,
    #[validate(range(min = 0.001, message = "La tasa de interés debe ser mayor a cero"))]
    pub interest_rate: Option<f64>,
    #[validate(custom = "validate_due_date_optional")]
    pub due_date: Option<String>,
    pub status: Option<String>,
}

fn validate_due_date_optional(date: &str) -> Result<(), validator::ValidationError> {
    validate_due_date(date)
}

impl UpdateLoanDto {
    pub fn parse_due_date(&self) -> Result<Option<NaiveDate>, String> {
        match &self.due_date {
            Some(date_str) => {
                let parsed = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .map_err(|_| "Formato de fecha inválido. Use YYYY-MM-DD".to_string())?;
                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }

    pub fn validate_status(status: &Option<String>) -> Result<(), String> {
        match status {
            Some(s) => {
                let valid_statuses = ["pendiente", "aprobado", "rechazado"];
                if valid_statuses.contains(&s.to_lowercase().as_str()) {
                    Ok(())
                } else {
                    Err(format!(
                        "Estado inválido. Debe ser uno de: {}",
                        valid_statuses.join(", ")
                    ))
                }
            }
            None => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanResponseDto {
    pub id: i64,
    pub amount: f64,
    pub interest_rate: f64,
    pub due_date: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl LoanResponseDto {
    pub fn from_loan(loan: &crate::models::loan::Loan) -> Self {
        LoanResponseDto {
            id: loan.id,
            amount: loan.amount,
            interest_rate: loan.interest_rate,
            due_date: loan.due_date.format("%Y-%m-%d").to_string(),
            status: loan.status.clone(),
            created_at: loan.created_at.format("%Y-%m-%d").to_string(),
            updated_at: loan.updated_at.format("%Y-%m-%d").to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanListResponseDto {
    pub loans: Vec<LoanResponseDto>,
    pub total: usize,
}

impl LoanListResponseDto {
    pub fn from_loans(loans: &[crate::models::loan::Loan]) -> Self {
        LoanListResponseDto {
            loans: loans.iter().map(LoanResponseDto::from_loan).collect(),
            total: loans.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponseDto {
    pub error: String,
    pub message: String,
    pub details: Option<Vec<String>>,
}

impl ErrorResponseDto {
    pub fn new(error: &str, message: &str) -> Self {
        ErrorResponseDto {
            error: error.to_string(),
            message: message.to_string(),
            details: None,
        }
    }

    pub fn with_details(error: &str, message: &str, details: Vec<String>) -> Self {
        ErrorResponseDto {
            error: error.to_string(),
            message: message.to_string(),
            details: Some(details),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponseDto<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
}

impl<T> SuccessResponseDto<T> {
    pub fn new(data: T) -> Self {
        SuccessResponseDto {
            success: true,
            data,
            message: None,
        }
    }

    pub fn with_message(data: T, message: &str) -> Self {
        SuccessResponseDto {
            success: true,
            data,
            message: Some(message.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_loan_dto_validation() {
        let dto = CreateLoanDto {
            amount: 1000.0,
            interest_rate: 0.1,
            due_date: "2025-12-31".to_string(),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_create_loan_dto_invalid_amount() {
        let dto = CreateLoanDto {
            amount: -100.0,
            interest_rate: 0.1,
            due_date: "2025-12-31".to_string(),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_create_loan_dto_invalid_interest_rate() {
        let dto = CreateLoanDto {
            amount: 1000.0,
            interest_rate: -0.1,
            due_date: "2025-12-31".to_string(),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_update_loan_dto_partial_update() {
        let dto = UpdateLoanDto {
            amount: Some(2000.0),
            interest_rate: None,
            due_date: None,
            status: None,
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_loan_response_dto_from_loan() {
        let future_date = chrono::Utc::now().date_naive() + chrono::Duration::days(30);
        let loan = crate::models::loan::Loan::new(1, 1000.0, 0.1, future_date).unwrap();
        let response = LoanResponseDto::from_loan(&loan);
        assert_eq!(response.id, 1);
        assert_eq!(response.amount, 1000.0);
    }

    #[test]
    fn test_error_response_dto_creation() {
        let error = ErrorResponseDto::new("VALIDATION_ERROR", "Error de validación");
        assert_eq!(error.error, "VALIDATION_ERROR");
    }

    #[test]
    fn test_success_response_dto_creation() {
        let response: SuccessResponseDto<String> = SuccessResponseDto::new("OK".to_string());
        assert!(response.success);
    }
}

// === ARCHIVO: Cargo.toml ===
[package]
name = "rust-actix-loan-api"
version = "1.0.0"
edition = "2021"
description = "REST API para gestión de préstamos con Rust y Actix Web"
license = "MIT"

[lib]
path = "src/lib.rs"

[[bin]]
name = "rust-actix-loan-api"
path = "src/main.rs"

[dependencies]
actix-web = "4.4.0"
actix-rt = "4.0.0"
diesel = { version = "2.1.0", features = ["postgres", "r2d2", "chrono", "serde_json"] }
diesel_migrations = "2.1.0"
serde = { version = "1.0.193", features = ["derive"] }
serde_json = "1.0.108"
validator = { version = "2.1.0", features = ["derive"] }
thiserror = "1.0.50"
anyhow = "1.0.75"
dotenv = "0.15.0"
tokio = { version = "1.35.0", features = ["full"] }
chrono = { version = "0.4.31", features = ["serde"] }
bcrypt = "0.15.0"
jsonwebtoken = "9.2.0"
r2d2 = "0.8.10"
r2d2_diesel = "1.0.0"
log = "0.4"
env_logger = "0.11.0"

[dev-dependencies]
actix-rt = "4.0.0"
actix-web = "4.4.0"

[features]
default = ["diesel/postgres"]

[profile.release]
opt-level = 3
lto = true
codegen-units = 1

// === ARCHIVO: src/main.rs ===
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use log::info;
use std::env;

mod controllers;
mod dtos;
mod errors;
mod models;
mod repositories;
mod services;

use controllers::loan_controller;
use errors::loan_error::AppError;
use services::loan_service::LoanService;

fn configure_app(cfg: &mut web::ServiceConfig) {
    let loan_service = web::Data::new(LoanService::new());
    
    cfg.app_data(loan_service)
        .service(
            web::scope("/api/v1")
                .service(
                    web::resource("/loans")
                        .route(web::get().to(loan_controller::get_all_loans))
                        .route(web::post().to(loan_controller::create_loan))
                )
                .service(
                    web::resource("/loans/{id}")
                        .route(web::get().to(loan_controller::get_loan_by_id))
                        .route(web::put().to(loan_controller::update_loan))
                        .route(web::delete().to(loan_controller::delete_loan))
                )
                .service(
                    web::resource("/loans/{id}/approve")
                        .route(web::post().to(loan_controller::approve_loan))
                )
                .service(
                    web::resource("/loans/{id}/reject")
                        .route(web::post().to(loan_controller::reject_loan))
                )
        )
        .route("/health", web::get().to(health_check));
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "loan-api",
        "version": "1.0.0"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    info!("Iniciando servidor de API de préstamos...");
    
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("El puerto debe ser un número válido");
    
    info!("Servidor escuchando en {}:{}", host, port);
    
    HttpServer::new(configure_app)
        .bind(("0.0.0.0", port))?
        .run()
        .await
}

// === ARCHIVO: src/controllers/loan_controller.rs ===
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use log::{error, info};

use crate::dtos::loan_dto::{
    CreateLoanDto, ErrorResponseDto, LoanResponseDto, SuccessResponseDto, UpdateLoanDto,
};
use crate::errors::loan_error::AppError;
use crate::models::loan::Loan;
use crate::services::loan_service::LoanService;

pub async fn get_all_loans(
    service: web::Data<LoanService>,
) -> Result<HttpResponse, AppError> {
    info!("Obteniendo todos los préstamos");
    
    match service.get_all().await {
        Ok(loans) => {
            let response = LoanResponseDto::from_loans(&loans);
            Ok(HttpResponse::Ok().json(SuccessResponseDto::with_message(
                response,
                "Préstamos obtenidos exitosamente",
            )))
        }
        Err(e) => {
            error!("Error al obtener préstamos: {}", e);
            Err(AppError::NotFound("No se encontraron préstamos".to_string()))
        }
    }
}

pub async fn get_loan_by_id(
    id: web::Path<i64>,
    service: web::Data<LoanService>,
) -> Result<HttpResponse, AppError> {
    let loan_id = id.into_inner();
    info!("Obteniendo préstamo con ID: {}", loan_id);
    
    match service.get_by_id(loan_id).await {
        Ok(Some(loan)) => {
            let response = LoanResponseDto::from_loan(&loan);
            Ok(HttpResponse::Ok().json(SuccessResponseDto::with_message(
                response,
                "Préstamo obtenido exitosamente",
            )))
        }
        Ok(None) => {
            error!("Préstamo no encontrado: {}", loan_id);
            Err(AppError::NotFound(format!(
                "Préstamo con ID {} no encontrado",
                loan_id
            )))
        }
        Err(e) => {
            error!("Error al obtener préstamo {}: {}", loan_id, e);
            Err(AppError::InternalServerError(
                "Error al procesar la solicitud".to_string(),
            ))
        }
    }
}

pub async fn create_loan(
    dto: web::Json<CreateLoanDto>,
    service: web::Data<LoanService>,
) -> Result<HttpResponse, AppError> {
    info!("Creando nuevo préstamo");
    
    if let Err(errors) = dto.validate_all() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::with_details(
            "VALIDATION_ERROR",
            "Error de validación",
            errors,
        )));
    }
    
    let due_date = dto.parse_due_date()?;
    
    let mut loan = Loan::new(
        dto.amount,
        dto.interest_rate,
        due_date,
    );
    
    match service.create(&mut loan).await {
        Ok(created) => {
            info!("Préstamo creado exitosamente con ID: {}", created.id);
            let response = LoanResponseDto::from_loan(&created);
            Ok(HttpResponse::Created().json(SuccessResponseDto::with_message(
                response,
                "Préstamo creado exitosamente",
            )))
        }
        Err(e) => {
            error!("Error al crear préstamo: {}", e);
            Err(AppError::InternalServerError(
                "Error al crear el préstamo".to_string(),
            ))
        }
    }
}

pub async fn update_loan(
    id: web::Path<i64>,
    dto: web::Json<UpdateLoanDto>,
    service: web::Data<LoanService>,
) -> Result<HttpResponse, AppError> {
    let loan_id = id.into_inner();
    info!("Actualizando préstamo con ID: {}", loan_id);
    
    if let Some(ref amount) = dto.amount {
        if *amount <= 0.0 {
            return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::new(
                "INVALID_AMOUNT",
                "El monto debe ser mayor a 0",
            )));
        }
    }
    
    if let Some(ref interest_rate) = dto.interest_rate {
        if *interest_rate < 0.0 {
            return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::new(
                "INVALID_INTEREST_RATE",
                "La tasa de interés no puede ser negativa",
            )));
        }
    }
    
    let existing = service.get_by_id(loan_id).await?;
    if existing.is_none() {
        return Err(AppError::NotFound(format!(
            "Préstamo con ID {} no encontrado",
            loan_id
        )));
    }
    
    let mut loan = existing.unwrap();
    
    if let Some(amount) = dto.amount {
        loan.update_amount(amount)?;
    }
    
    if let Some(interest_rate) = dto.interest_rate {
        loan.update_interest_rate(interest_rate)?;
    }
    
    if let Some(ref due_date_str) = dto.due_date {
        let new_due_date = chrono::NaiveDate::parse_from_str(
            due_date_str,
            "%Y-%m-%d",
        )
        .map_err(|_| AppError::BadRequest("Formato de fecha inválido. Use YYYY-MM-DD".to_string()))?;
        
        if new_due_date <= chrono::Utc::now().date_naive() {
            return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::new(
                "INVALID_DUE_DATE",
                "La fecha de vencimiento debe ser posterior a hoy",
            )));
        }
        
        loan.update_due_date(new_due_date)?;
    }
    
    match service.update(&loan).await {
        Ok(updated) => {
            info!("Préstamo {} actualizado exitosamente", loan_id);
            let response = LoanResponseDto::from_loan(&updated);
            Ok(HttpResponse::Ok().json(SuccessResponseDto::with_message(
                response,
                "Préstamo actualizado exitosamente",
            )))
        }
        Err(e) => {
            error!("Error al actualizar préstamo {}: {}", loan_id, e);
            Err(AppError::InternalServerError(
                "Error al actualizar el préstamo".to_string(),
            ))
        }
    }
}

pub async fn delete_loan(
    id: web::Path<i64>,
    service: web::Data<LoanService>,
) -> Result<HttpResponse, AppError> {
    let loan_id = id.into_inner();
    info!("Eliminando préstamo con ID: {}", loan_id);
    
    let existing = service.get_by_id(loan_id).await?;
    if existing.is_none() {
        return Err(AppError::NotFound(format!(
            "Préstamo con ID {} no encontrado",
            loan_id
        )));
    }
    
    match service.delete(loan_id).await {
        Ok(_) => {
            info!("Préstamo {} eliminado exitosamente", loan_id);
            Ok(HttpResponse::NoContent().finish())
        }
        Err(e) => {
            error!("Error al eliminar préstamo {}: {}", loan_id, e);
            Err(AppError::InternalServerError(
                "Error al eliminar el préstamo".to_string(),
            ))
        }
    }
}

pub async fn approve_loan(
    id: web::Path<i64>,
    service: web::Data<LoanService>,
) -> Result<HttpResponse, AppError> {
    let loan_id = id.into_inner();
    info!("Aprobando préstamo con ID: {}", loan_id);
    
    let existing = service.get_by_id(loan_id).await?;
    if existing.is_none() {
        return Err(AppError::NotFound(format!(
            "Préstamo con ID {} no encontrado",
            loan_id
        )));
    }
    
    let mut loan = existing.unwrap();
    
    if loan.is_approved() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::new(
            "ALREADY_APPROVED",
            "El préstamo ya está aprobado",
        )));
    }
    
    if loan.is_rejected() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::new(
            "ALREADY_REJECTED",
            "No se puede aprobar un préstamo rechazado",
        )));
    }
    
    loan.approve();
    
    match service.update(&loan).await {
        Ok(updated) => {
            info!("Préstamo {} aprobado exitosamente", loan_id);
            let response = LoanResponseDto::from_loan(&updated);
            Ok(HttpResponse::Ok().json(SuccessResponseDto::with_message(
                response,
                "Préstamo aprobado exitosamente",
            )))
        }
        Err(e) => {
            error!("Error al aprobar préstamo {}: {}", loan_id, e);
            Err(AppError::InternalServerError(
                "Error al aprobar el préstamo".to_string(),
            ))
        }
    }
}

pub async fn reject_loan(
    id: web::Path<i64>,
    service: web::Data<LoanService>,
) -> Result<HttpResponse, AppError> {
    let loan_id = id.into_inner();
    info!("Rechazando préstamo con ID: {}", loan_id);
    
    let existing = service.get_by_id(loan_id).await?;
    if existing.is_none() {
        return Err(AppError::NotFound(format!(
            "Préstamo con ID {} no encontrado",
            loan_id
        )));
    }
    
    let mut loan = existing.unwrap();
    
    if loan.is_rejected() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::new(
            "ALREADY_REJECTED",
            "El préstamo ya está rechazado",
        )));
    }
    
    if loan.is_approved() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponseDto::new(
            "ALREADY_APPROVED",
            "No se puede rechazar un préstamos aprobado",
        )));
    }
    
    loan.reject();
    
    match service.update(&loan).await {
        Ok(updated) => {
            info!("Préstamo {} rechazado exitosamente", loan_id);
            let response = LoanResponseDto::from_loan(&updated);
            Ok(HttpResponse::Ok().json(SuccessResponseDto::with_message(
                response,
                "Préstamo rechazado exitosamente",
            )))
        }
        Err(e) => {
            error!("Error al rechazar préstamo {}: {}", loan_id, e);
            Err(AppError::InternalServerError(
                "Error al rechazar el préstamo".to_string(),
            ))
        }
    }
}

// === ARCHIVO: src/errors/loan_error.rs ===
use actix_web::{http::StatusCode, ResponseError};
use std::fmt;

#[derive(Debug) Clone)]
pub enum LoanError {
    NotFound(String),
    Validation(String),
    Repository(String),
    Service(String),
}

impl fmt::Display for LoanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoanError::NotFound(msg) => write!(f, "Préstamo no encontrado: {}", msg),
            LoanError::Validation(msg) => write!(f, "Error de validación: {}", msg),
            LoanError::Repository(msg) => write!(f, "Error de base de datos: {}", msg),
            LoanError::Service(msg) => write!(f, "Error en el servicio: {}", msg),
        }
    }
}

impl std::error::Error for LoanError {}

impl ResponseError for LoanError {
    fn status_code(&self) -> StatusCode {
        match self {
            LoanError::NotFound(_) => StatusCode::NOT_FOUND,
            LoanError::Validation(_) => StatusCode::BAD_REQUEST,
            LoanError::Repository(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LoanError::Service(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let status = self.status_code();
        let message = self.to_string();
        actix_web::HttpResponse::build(status)
            .json(serde_json::json!({
                "error": self.error_type(),
                "message": message
            }))
    }
}

impl LoanError {
    fn error_type(&self) -> &'static str {
        match self {
            LoanError::NotFound(_) => "NOT_FOUND",
            LoanError::Validation(_) => "VALIDATION_ERROR",
            LoanError::Repository(_) => "REPOSITORY_ERROR",
            LoanError::Service(_) => "SERVICE_ERROR",
        }
    }

    pub fn not_found(id: &str) -> Self {
        LoanError::NotFound(format!("No existe un préstamo con ID: {}", id))
    }

    pub fn validation(message: impl Into<String>) -> Self {
        LoanError::Validation(message.into())
    }

    pub fn repository(message: impl Into<String>) -> Self {
        LoanError::Repository(message.into())
    }

    pub fn service(message: impl Into<String>) -> Self {
        LoanError::Service(message.into())
    }
}

impl From<diesel::result::Error> for LoanError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => {
                LoanError::Repository("Registro no encontrado en la base de datos".to_string())
            }
            _ => LoanError::Repository(format!("Error de base de datos: {}", err)),
        }
    }
}

impl From<diesel::dsl::EqAllErrors> for LoanError {
    fn from(_err: diesel::dsl::EqAllErrors) -> Self {
        LoanError::Repository("Error al construir consulta".to_string())
    }
}

impl serde::Serialize for LoanError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// === ARCHIVO: src/repositories/loan_repository.rs ===
use crate::errors::loan_error::LoanError;
use crate::models::loan::Loan;
use diesel::prelude::*;
use diesel::PgConnection;
use std::sync::Arc;

pub trait LoanRepository: Send + Sync {
    fn find_all(&self) -> Result<Vec<Loan>, LoanError>;
    fn find_by_id(&self, id: i64) -> Result<Loan, LoanError>;
    fn create(&self, loan: &Loan) -> Result<Loan, LoanError>;
    fn update(&self, id: i64, loan: &Loan) -> Result<Loan, LoanError>;
    fn delete(&self, id: i64) -> Result<(), LoanError>;
    fn find_pending(&self) -> Result<Vec<Loan>, LoanError>;
    fn find_overdue(&self) -> Result<Vec<Loan>, LoanError>;
}

pub struct DieselLoanRepository {
    connection: Arc<PgConnection>,
}

impl DieselLoanRepository {
    pub fn new(connection: Arc<PgConnection>) -> Self {
        Self { connection }
    }
}

impl LoanRepository for DieselLoanRepository {
    fn find_all(&self) -> Result<Vec<Loan>, LoanError> {
        use crate::schema::loans::dsl::*;

        let result = loans
            .load::<Loan>(&*self.connection)
            .map_err(LoanError::from)?;

        Ok(result)
    }

    fn find_by_id(&self, id: i64) -> Result<Loan, LoanError> {
        use crate::schema::loans::dsl::*;

        loans
            .filter(id.eq(id))
            .first::<Loan>(&*self.connection)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => LoanError::not_found(&id.to_string()),
                _ => LoanError::from(e),
            })
    }

    fn create(&self, loan: &Loan) -> Result<Loan, LoanError> {
        use crate::schema::loans;

        diesel::insert_into(loans::table)
            .values(loan)
            .get_result(&*self.connection)
            .map_err(LoanError::from)
    }

    fn update(&self, id: i64, loan: &Loan) -> Result<Loan, LoanError> {
        use crate::schema::loans::dsl::*;

        let rows_updated = diesel::update(loans.filter(id.eq(id)))
            .set(loan)
            .execute(&*self.connection)
            .map_err(LoanError::from)?;

        if rows_updated == 0 {
            return Err(LoanError::not_found(&id.to_string()));
        }

        self.find_by_id(id)
    }

    fn delete(&self, id: i64) -> Result<(), LoanError> {
        use crate::schema::loans::dsl::*;

        let rows_deleted = diesel::delete(loans.filter(id.eq(id)))
            .execute(&*self.connection)
            .map_err(LoanError::from)?;

        if rows_deleted == 0 {
            return Err(LoanError::not_found(&id.to_string()));
        }

        Ok(())
    }

    fn find_pending(&self) -> Result<Vec<Loan>, LoanError> {
        use crate::schema::loans::dsl::*;

        loans
            .filter(status.eq("pending"))
            .load::<Loan>(&*self.connection)
            .map_err(LoanError::from)
    }

    fn find_overdue(&self) -> Result<Vec<Loan>, LoanError> {
        use crate::models::loan::LoanStatus;
        use crate::schema::loans::dsl::*;
        use chrono::Utc;

        let today = Utc::now().date_naive();

        let result = loans
            .filter(status.eq(LoanStatus::Approved.to_string()))
            .filter(due_date.lt(today))
            .load::<Loan>(&*self.connection)
            .map_err(LoanError::from)?;

        Ok(result)
    }
}

// === ARCHIVO: src/services/loan_service.rs ===
use crate::dtos::loan_dto::{CreateLoanDto, LoanResponseDto, UpdateLoanDto};
use crate::errors::loan_error::LoanError;
use crate::models::loan::Loan;
use crate::repositories::loan_repository::LoanRepository;
use chrono::Utc;
use std::sync::Arc;

pub struct LoanService {
    repository: Arc<dyn LoanRepository>,
}

impl LoanService {
    pub fn new(repository: Arc<dyn LoanRepository>) -> Self {
        Self { repository }
    }

    pub fn get_all_loans(&self) -> Result<Vec<LoanResponseDto>, LoanError> {
        let loans = self.repository.find_all()?;
        Ok(LoanResponseDto::from_loans(&loans))
    }

    pub fn get_loan_by_id(&self, id: i64) -> Result<LoanResponseDto, LoanError> {
        let loan = self.repository.find_by_id(id)?;
        Ok(LoanResponseDto::from_loan(&loan))
    }

    pub fn create_loan(&self, dto: CreateLoanDto) -> Result<LoanResponseDto, LoanError> {
        dto.validate_all().map_err(LoanError::validation)?;

        let due_date = dto.parse_due_date()
            .map_err(|e| LoanError::validation(e))??;

        if due_date <= Utc::now().date_naive() {
            return Err(LoanError::validation(
                "La fecha de vencimiento debe ser posterior a la fecha actual"
            ));
        }

        let loan = Loan::new(
            dto.amount,
            dto.interest_rate,
            due_date,
        );

        let created = self.repository.create(&loan)?;
        Ok(LoanResponseDto::from_loan(&created))
    }

    pub fn update_loan(&self, id: i64, dto: UpdateLoanDto) -> Result<LoanResponseDto, LoanError> {
        let existing = self.repository.find_by_id(id)?;

        let mut updated = existing.clone();

        if let Some(amount) = dto.amount {
            if amount <= 0.0 {
                return Err(LoanError::validation(
                    "El monto debe ser un número positivo"
                ));
            }
            updated.update_amount(amount)
                .map_err(LoanError::validation)?;
        }

        if let Some(interest_rate) = dto.interest_rate {
            if interest_rate < 0.0 {
                return Err(LoanError::validation(
                    "La tasa de interés no puede ser negativa"
                ));
            }
            updated.update_interest_rate(interest_rate)
                .map_err(LoanError::validation)?;
        }

        if let Some(due_date_str) = & dto.due_date {
            if let Ok(Some(new_due_date)) = dto.parse_due_date() {
                if new_due_date <= Utc::now().date_naive() {
                    return Err(LoanError::validation(
                        "La fecha de vencimiento debe ser posterior a la fecha actual"
                    ));
                }
                updated.update_due_date(new_due_date)
                    .map_err(LoanError::validation)?;
            }
        }

        if let Some(status) = dto.status {
            match status.as_str() {
                "approved" => updated.approve(),
                "rejected" => updated.reject(),
                "pending" => {}
                _ => return Err(LoanError::validation(
                    "Estado inválido. Debe ser: pending, approved o rejected"
                )),
            }
        }

        let result = self.repository.update(id, &updated)?;
        Ok(LoanResponseDto::from_loan(&result))
    }

    pub fn delete_loan(&self, id: i64) -> Result<(), LoanError> {
        self.repository.find_by_id(id)?;
        self.repository.delete(id)
    }

    pub fn approve_loan(&self, id: i64) -> Result<LoanResponseDto, LoanError> {
        let mut loan = self.repository.find_by_id(id)?;

        if !loan.is_pending() {
            return Err(LoanError::validation(
                "Solo se pueden aprobar préstamos en estado pendiente"
            ));
        }

        loan.approve();
        let updated = self.repository.update(id, &loan)?;
        Ok(LoanResponseDto::from_loan(&updated))
    }

    pub fn reject_loan(&self, id: i64) -> Result<LoanResponseDto, LoanError> {
        let mut loan = self.repository.find_by_id(id)?;

        if !loan.is_pending() {
            return Err(LoanError::validation(
                "Solo se pueden rechazar préstamos en estado pendiente"
            ));
        }

        loan.reject();
        let updated = self.repository.update(id, &loan)?;
        Ok(LoanResponseDto::from_loan(&updated))
    }

    pub fn get_pending_loans(&self) -> Result<Vec<LoanResponseDto>, LoanError> {
        let loans = self.repository.find_pending()?;
        Ok(LoanResponseDto::from_loans(&loans))
    }

    pub fn get_overdue_loans(&self) -> Result<Vec<LoanResponseDto>, LoanError> {
        let loans = self.repository.find_overdue()?;
        Ok(LoanResponseDto::from_loans(&loans))
    }

    pub fn calculate_total_amount(&self, id: i64) -> Result<f64, LoanError> {
        let loan = self.repository.find_by_id(id)?;
        Ok(loan.calculate_total_amount())
    }

    pub fn check_overdue(&self, id: i64) -> Result<bool, LoanError> {
        let loan = self.repository.find_by_id(id)?;
        Ok(loan.is_overdue())
    }
}

// === ARCHIVO: src/tests/loan_controller_tests.rs ===
use actix_web::{test, web, App, http::StatusCode};
use serde_json::json;

mod controller_integration_tests {
    use super::*;

    #[actix_web::test]
    async fn test_create_loan_success() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let payload = json!({
            "amount": 10000.0,
            "interest_rate": 5.5,
            "due_date": "2025-12-31"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn test_create_loan_invalid_amount() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let payload = json!({
            "amount": -1000.0,
            "interest_rate": 5.5,
            "due_date": "2025-12-31"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_create_loan_invalid_interest_rate() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let payload = json!({
            "amount": 10000.0,
            "interest_rate": -2.0,
            "due_date": "2025-12-31"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_create_loan_past_due_date() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let payload = json!({
            "amount": 10000.0,
            "interest_rate": 5.5,
            "due_date": "2020-01-01"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_all_loans_success() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/loans")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_loan_by_id_success() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let create_payload = json!({
            "amount": 15000.0,
            "interest_rate": 4.0,
            "due_date": "2025-06-30"
        });

        let create_req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(create_payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let create_resp = test::call_service(&app, create_req).await;
        assert_eq!(create_resp.status(), StatusCode::CREATED);

        let req = test::TestRequest::get()
            .uri("/api/loans/1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_loan_by_id_not_found() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/loans/99999")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_update_loan_success() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let create_payload = json!({
            "amount": 10000.0,
            "interest_rate": 3.5,
            "due_date": "2025-09-30"
        });

        let create_req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(create_payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let _create_resp = test::call_service(&app, create_req).await;

        let update_payload = json!({
            "amount": 12000.0,
            "interest_rate": 4.5
        });

        let req = test::TestRequest::put()
            .uri("/api/loans/1")
            .set_payload(update_payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_update_loan_not_found() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let payload = json!({
            "amount": 20000.0,
            "interest_rate": 6.0
        });

        let req = test::TestRequest::put()
            .uri("/api/loans/88888")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_delete_loan_success() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let create_payload = json!({
            "amount": 5000.0,
            "interest_rate": 2.0,
            "due_date": "2025-03-31"
        });

        let create_req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(create_payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let _create_resp = test::call_service(&app, create_req).await;

        let req = test::TestRequest::delete()
            .uri("/api/loans/1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn test_delete_loan_not_found() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let req = test::TestRequest::delete()
            .uri("/api/loans/77777")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_approve_loan_success() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let create_payload = json!({
            "amount": 8000.0,
            "interest_rate": 3.0,
            "due_date": "2025-08-31"
        });

        let create_req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(create_payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let _create_resp = test::call_service(&app, create_req).await;

        let req = test::TestRequest::put()
            .uri("/api/loans/1/approve")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_reject_loan_success() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let create_payload = json!({
            "amount": 25000.0,
            "interest_rate": 7.5,
            "due_date": "2025-11-30"
        });

        let create_req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(create_payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let _create_resp = test::call_service(&app, create_req).await;

        let req = test::TestRequest::put()
            .uri("/api/loans/1/reject")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_missing_required_fields() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let payload = json!({
            "amount": 10000.0
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_invalid_json_format() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload("not valid json")
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
```
