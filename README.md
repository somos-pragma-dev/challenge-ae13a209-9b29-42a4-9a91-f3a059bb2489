# Desarrollo de API REST con Rust y Actix Web

Debes construir una API REST para un sistema de gestión de préstamos en una fintech. La API debe permitir la creación, lectura, actualización y eliminación de préstamos. Los préstamos tienen los siguientes atributos: id, monto, tasa de interés, fecha de vencimiento, estado (pendiente, aprobado, rechazado). La API debe validar que el monto y la tasa de interés sean números positivos, y que la fecha de vencimiento sea posterior a la fecha actual. Además, debe manejar adecuadamente los errores de validación y devolver mensajes de error claros al cliente.

## Informacion General

| Campo | Valor |
|-------|-------|
| **Tema** | rust-actix-web |
| **Nivel** | junior-l2 |
| **Tipo** | practical |
| **Tiempo estimado** | 8 horas |

## Fases del Reto

### Fase 0: Configuración del Proyecto

**Objetivo:** Obtener el proyecto base funcional enviando el Código Base a un asistente de IA, que lo analizará, corregirá errores y generará un ZIP listo para usar.

**Tiempo estimado:** 15-30 minutos

**Instrucciones:**

- Asegúrate de tener instalado para ejecutar el proyecto: Node.js 18+, npm, VS Code o similar.
- Copia todo el contenido del campo **Código Base** de este reto — incluyendo el texto de instrucciones que aparece al inicio.
- Abre un asistente de IA (Claude en claude.ai, ChatGPT o Gemini — se recomienda Claude), pega el contenido copiado en el chat y envíalo.
- El asistente analizará los archivos, corregirá errores y generará un archivo ZIP descargable. Descárgalo y extráelo en la carpeta donde quieras trabajar.
- Ejecuta `npm install && npm run build` (o `npm start`). Si no hay errores, estás listo.

**Entregable:** El proyecto compila/arranca sin errores.

<details>
<summary>Pistas de conocimiento</summary>

- Copia el Código Base completo incluyendo el texto de instrucciones al inicio — esas instrucciones le indican al asistente exactamente qué hacer con los archivos.
- Si el asistente no genera el ZIP automáticamente al terminar el análisis, escríbele: "genera el ZIP ahora".
- Si el proyecto tiene errores al arrancar, comparte el mensaje de error con el mismo asistente para que lo corrija.

</details>

### Fase 1: Definición del modelo de datos

**Objetivo:** Definir el modelo de datos para los préstamos, incluyendo sus atributos y validaciones.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- Identifica los atributos necesarios para representar un préstamo.
- Define las validaciones para cada atributo.
- Crea un modelo de datos que incluya los atributos y las validaciones definidas.

**Entregable:** Modelo de datos para préstamos con atributos y validaciones definidas.

<details>
<summary>Pistas de conocimiento</summary>

- Considera los diferentes tipos de datos que puedes usar para representar cada atributo.
- Piensa en las validaciones que necesitas para asegurar la integridad de los datos.

</details>

### Fase 2: Implementación de endpoints

**Objetivo:** Implementar los endpoints para crear, leer, actualizar y eliminar préstamos.

**Tiempo estimado:** 4 horas

**Instrucciones:**

- Define los endpoints necesarios para crear, leer, actualizar y eliminar préstamos.
- Implementa la lógica para cada endpoint, incluyendo las validaciones definidas en la fase anterior.
- Maneja adecuadamente los errores de validación y devuelva mensajes de error claros al cliente.

**Entregable:** Endpoints implementados para crear, leer, actualizar y eliminar préstamos, con validaciones y manejo de errores.

<details>
<summary>Pistas de conocimiento</summary>

- Considera el uso de métodos HTTP adecuados para cada operación (POST para crear, GET para leer, PUT para actualizar, DELETE para eliminar).
- Piensa en cómo puedes devolver mensajes de error claros y útiles al cliente.

</details>

### Fase 3: Pruebas y validación

**Objetivo:** Realizar pruebas y validación de la API implementada.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- Crea casos de prueba para cada endpoint, incluyendo casos de éxito y casos de error.
- Ejecuta las pruebas y verifica que la API se comporte como se espera en cada caso.
- Documenta los resultados de las pruebas y cualquier problema encontrado.

**Entregable:** Casos de prueba ejecutados y documentados, con resultados y problemas encontrados.

<details>
<summary>Pistas de conocimiento</summary>

- Considera el uso de herramientas de prueba como Postman o curl para ejecutar las pruebas.
- Piensa en cómo puedes documentar los resultados de las pruebas de manera clara y concisa.

</details>

## Dimensiones Evaluadas

- **queEs**: ¿Qué es un modelo de datos y por qué es importante en el desarrollo de una API?
- **paraQueSirve**: ¿Para qué sirven los endpoints en una API y cómo se relacionan con el modelo de datos?
- **comoSeUsa**: ¿Cómo se usan las validaciones en una API y por qué son importantes?
- **erroresComunes**: ¿Cuáles son los errores comunes al implementar una API y cómo se pueden evitar?

## Criterios de Evaluacion

- Definición clara del modelo de datos para préstamos con atributos y validaciones.
- Implementación correcta de endpoints para crear, leer, actualizar y eliminar préstamos, incluyendo validaciones y manejo de errores.
- Ejecución y documentación de casos de prueba para cada endpoint, con resultados y problemas encontrados.

## Como trabajar con un asistente de IA

Hay dos caminos, elegi uno:

- **AGENTS.md** (recomendado) — instrucciones nativas del repo. Abri esta carpeta con tu agente local (Claude Code, Cursor, Codex, Copilot, Gemini) y las carga solo. Sabe que archivos faltan y con que comando se verifica, y completa el scaffold escribiendo en disco.
- **PROMPT_MEJORA.md** — para copiar y pegar en un chat (claude.ai, ChatGPT). Devuelve un ZIP con el proyecto. Sirve si no tenes un agente en el IDE.

Ninguno de los dos resuelve las fases del reto: eso es tu trabajo.

## Verificacion

El proyecto esta listo para trabajar cuando este comando corre sin errores:

```bash
el comando de build o arranque canonico del stack elegido
```

---

*Reto generado automaticamente por Challenge Generator - Pragma*
