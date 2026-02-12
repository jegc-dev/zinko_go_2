# ZinkoGo 2.0 (ITAM Predictivo)

**Descripción:** Agente nativo en Rust que monitorea el estado de SSDs, ciclos de batería y telemetría térmica a nivel de Kernel. Dispara eventos automáticos al GLPI de ZINKO para reemplazos preventivos antes de que ocurra la falla.

**Beneficio:** Minimiza tiempos de inactividad, reduce costos de soporte reactivo y extiende la vida útil de los activos.

---

## 🛠️ Requisitos del Sistema

### 💻 Entorno de Desarrollo
- **Lenguaje:** Rust (Edición 2021+).
- **SDK/Herramientas:** Cargo, Compilador de Rust (rustc).
- **Librerías Clave (Crates):** 
  - `sysinfo`: Métricas generales (CPU, RAM).
  - `battery`: Ciclos de carga y salud profunda de batería.
  - `hdd` o `smart-lib`: Datos S.M.A.R.T. de SSDs/HDDs.
  - `sensors`: Temperaturas de placa y ventiladores (lm-sensors).
  - `egui`: Interfaz de usuario inmediata y liviana.
  - `rumqttc`: Comunicación MQTT robusta.
- **Entorno:** Multiplataforma (Windows 10/11 y Linux) para máxima flexibilidad en la demo.

### 🛰️ Capacidades de Monitoreo (Agente)
- **Almacenamiento:** Salud de SSDs (S.M.A.R.T), temperatura de discos.
- **Energía:** Ciclos de carga de batería, capacidad actual vs. original.
- **Térmica:** Sensores de CPU/GPU en tiempo real.
- **Integraciones:** Envío de eventos a GLPI y notificaciones vía Webhooks.

---

## 🧪 DEMO

### 1. El Agente Local: "The Data Gatherer" (Rust)
Crea un ejecutable en Rust que utilice el crate `sysinfo` para extraer métricas reales del hardware.

- **Misión:** Leer la salud del SSD, ciclos de batería y temperatura cada 5 segundos.

> [!IMPORTANT]
> **Disparador del evento en la Demo (Modo Simulación):**
> El agente monitorea la existencia de archivos "trigger" en su carpeta raíz para forzar estados de falla sin dañar el hardware:
> - `fail_disk.trigger`: La salud del SSD cae un 1% por segundo.
> - `fail_battery.trigger`: Los ciclos de batería suben a +1000 instantáneamente.
> - `fail_temp.trigger`: La temperatura sube a 90°C de forma sostenida.
> *Esto permite demostrar la reacción del motor predictivo y las alertas en vivo (Slack/Discord) de forma controlada.*

### 2. La App de Transparencia: "The Trust Builder"
Utiliza `egui` o `iced` (librerías de Rust para UI) para crear una ventana minimalista.

**Componentes Visuales:**
- Un gráfico de líneas en tiempo real con la temperatura.
- Un panel de "Consola de Privacidad" que muestre el JSON crudo que se está enviando a la nube.
- Un botón de "Auditar Conexión" que abra el navegador en la documentación del EULA. 
  > *Este botón garantiza transparencia total al dirigir al usuario a la política de privacidad, detallando qué métricas de hardware se recolectan y asegurando que no se accede a información personal ni archivos.*


### 3. El Cerebro en la Nube: "The Predictive Engine" (GCP)
Para la demo, utiliza una **Cloud Function** sencilla:

1. **Ingesta:** Recibe el JSON del agente vía HTTP o Pub/Sub.
2. **Lógica Predictiva (Heurística):**
   - Si `Temperatura > 85°C` por más de 10 segundos $\rightarrow$ **Alerta Crítica**.
   - Si `Ciclos_Bateria > 1000` $\rightarrow$ **Alerta de Reemplazo**.
3. **Salida:** En lugar de GLPI, usa un webhook de Discord o Slack. Cuando se inyecte el fallo, el equipo de soporte recibirá una notificación instantánea.

---

## 🛠️ Resumen Tecnológico

| Capa | Herramienta | Razón |
| :--- | :--- | :--- |
| **Backend Agente** | Rust + `sysinfo` / `battery` / `hdd` | Acceso granular a telemetría de bajo nivel. |
| **Interfaz Local** | `egui` (Rust) | Binario único, liviano y sin dependencias externas. |
| **Comunicación** | MQTT (`rumqttc`) | Estándar industrial para telemetría en tiempo real. |
| **Dashboard** | Grafana (Cloud) | Visualización profesional de series temporales. |

---

## 💰 Recursos y Costos Estimados

| Categoría | Concepto | Tiempo de Desarrollo | Costo Estimado (USD / COP) | Notas |
| :--- | :--- | :--- | :--- | :--- |
| **Infraestructura Cloud** | Google Cloud Functions (GCP) | N/A | $0.00 (Free Tier) | Ingesta y lógica de alertas. |
| **Infraestructura Cloud** | Google Artifact Registry | N/A | $0.00 (Free Tier) | Almacenamiento de binarios. |
| **Monitoreo** | Grafana Cloud | N/A | $0.00 (Free Tier) | Visualización (14 días retención). |
| **Desarrollo** | Licencias (Rust) | N/A | $0.00 | Open Source. |
| **Talento Humano** | Senior Rust Developer | 2 - 3 Semanas | $1,200 - $2,000 / **4.4M - 7.3M COP** | Sprint ágil para MVP funcional: Agente + UI + Cloud Alerting. |

> [!NOTE]
> Los costos en COP están calculados a una tasa de cambio aproximada de **1 USD ≈ 3,660 COP**. El tiempo de desarrollo contempla desde el diseño del agente hasta la integración final de alertas.

