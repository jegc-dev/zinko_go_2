# Plan de Trabajo: Demo Zinko Go 2.0 (Sprint de 3 Semanas)

Este plan detalla el desarrollo paso a paso del MVP para garantizar una demo técnica robusta y visualmente impactante.

---

## 📅 Cronograma de Desarrollo

### 🛠️ Semana 1: El Corazón del Agente (Backend y Telemetría)
*El objetivo es lograr que el código hable directamente con el hardware.*

1.  **Estructura Base:** Inicializar proyecto en Rust y configurar las crates principales (`sysinfo`, `battery`, `smart-lib`, `serde`).
2.  **Módulo de Telemetría:**
    *   Implementar lectura de salud SSD (S.M.A.R.T).
    *   Implementar lectura de ciclos de carga y capacidad de batería.
    *   Implementar monitoreo de temperatura de CPU/GPU.
3.  **Motor de Simulación (Gatillos):**
    *   Crear el observador de archivos (*File Watcher*) que detecte la creación de archivos `fail_disk.trigger`, `fail_battery.trigger` y `fail_temp.trigger`.
    *   Implementar la lógica de manipulación de datos en memoria para simular los fallos cuando estos archivos existan.
4.  **Pipeline de Datos:** Crear la estructura JSON centralizada para el transporte de métricas.

### 🖼️ Semana 2: El Espejo de Transparencia (UI y UX Local)
*El objetivo es hacer visible el trabajo del agente para generar confianza.*

1.  **Framework UI:** Integrar la librería `egui` para una interfaz nativa, ligera y sin dependencias.
2.  **Dashboard de Telemetría:**
    *   Diseñar el gráfico de líneas en tiempo real para la temperatura.
    *   Crear indicadores visuales (ProgressBar) para la salud del disco y batería.
3.  **Consola de Privacidad:**
    *   Implementar una ventana de texto que formatee el JSON de salida en tiempo real.
    *   Agregar el botón de **"Auditar Conexión"** que abra el navegador en la URL del EULA/Privacidad.
4.  **Estado de Conexión:** Indicador visual (Luz Verde/Roja) del estado de la comunicación con la nube.

### ☁️ Semana 3: La Reacción en la Nube (GCP y Alertas)
*El objetivo es cerrar el ciclo: detectar un problema y avisar al soporte.*

1.  **Ingesta Cloud:**
    *   Configurar una **Cloud Function** en Google Cloud para recibir los datos vía HTTP.
    *   Validar la recepción y formateo de los datos en la nube.
2.  **Motor de Reglas Simple:**
    *   Lógica en la nube: `Si Temp > 90 por 10s` o `Salud < 10%`, activar evento.
3.  **Canal de Alertas:**
    *   Configurar Webhook de Discord o Slack.
    *   Implementar el envío de la tarjeta de alerta enriquecida (Nombre del equipo, tipo de falla, prioridad).
4.  **Pruebas de Estrés y Demo:**
    *   Validación final: Ejecutar el agente, crear un archivo `.trigger` y cronometrar cuánto tarda en llegar la alerta a Discord (Meta: < 3 segundos).
5.  **Compilación Final:** Generar binario estático optimizado para Windows.

---

## 📦 Lista de Componentes a Desarrollar

| Componente | Especificación Técnica |
| :--- | :--- |
| **Colector de Hardware** | Código Rust nativo que usa llamadas al sistema (Kernel level). |
| **Simulador de Fallas** | Sistema de inyección de errores controlado vía archivos de texto. |
| **Ventana Resident** | Interfaz UI de 640x480 que corre en segundo plano. |
| **Logic Cloud** | Función serverless en Python/Node que hace de "cerebro" remoto. |
| **Integrador Alertas** | Adaptador para Webhooks de plataformas de chat. |

---

## 📈 ¿Qué constituye el éxito de la demo?
La demo se considera exitosa si, al ejecutar un archivo **trigger**, el equipo de soporte recibe una alerta con el diagnóstico correcto en tiempo real, mientras el usuario puede ver exactamente qué datos se están compartiendo desde su consola local.
