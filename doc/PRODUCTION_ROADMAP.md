# 🚀 Zinko Go 2.0 - Hoja de Ruta hacia Producción (ITAM Predictivo con AI)

Este documento detalla los requerimientos técnicos y funcionales necesarios para transformar el prototipo actual (Demo) en una solución empresarial de Gestión de Activos de TI (ITAM) potenciada por **Inteligencia Artificial** e integrada con el **GLPI de ZINKO**.

---

## 1. Seguridad y Autenticación (Prioridad Alta)
El prototipo actual utiliza webhooks abiertos y variables de entorno simples. Para producción se requiere:
- **Autenticación MDM/Agente:** Implementar mTLS (Mutual TLS) o tokens JWT firmados para que solo los agentes autorizados puedan enviar datos al ecosistema Zinko.
- **Gestión de Secretos:** Migrar las credenciales de acceso a la API de **GLPI** y otras llaves del código a **Google Cloud Secret Manager**.
- **Cifrado en Reposo y Tránsito:** Asegurar que los datos de telemetría viajen cifrados (HTTPS/TLS) y que cualquier persistencia en base de datos use llaves gestionadas (KMS).

## 2. Integración de Hardware Real y Telemetría
Actualmente, el agente utiliza valores "fallback" y simuladores.
- **Acceso a Sensores Nativo:** Implementar el uso completo de `libsensors` (Linux) y `WMI` (Windows) junto con `smart-lib` para obtener salud real de SSD/HDD y ciclos de batería profundos.
- **Protocolo Industrial (MQTT):** Migrar de HTTP simple a **MQTT (`rumqttc`)** para una comunicación de telemetría más eficiente, robusta y con soporte nativo para estados de conexión (Last Will and Testament).

## 3. Infraestructura e Inteligencia Artificial (GCP + AI)
- **Motor de Inferencia Predictiva:** Evolucionar de una lógica heurística (IF/THEN) a un modelo de **Machine Learning** auto-entrenado.
- **Vertex AI:** Utilizar los datos históricos en BigQuery para entrenar modelos que identifiquen patrones sutiles de degradación que la lógica simple no puede detectar (ej: correlación entre picos de temperatura y desgaste acelerado de celdas SSD).
- **Zinko Cloud Brain:** El servicio en la nube actuará como un "Data Scientist" automatizado que valida cada alerta antes de crear un ticket en GLPI, reduciendo falsos positivos.
- **Dashboard de Series Temporales:** Integrar **Grafana Cloud** para visualizar las predicciones de la IA sobre el tiempo de vida remanente (RUL - Remaining Useful Life) de cada activo.

## 4. Robustez del Agente (Hardenización)
- **Manejo de Errores y Reintentos:** Implementar una cola local de persistencia para asegurar que los datos no se pierdan durante caídas de red.
- **Auto-actualización (OTA):** Mecanismo seguro para actualizar el binario del agente Rust de forma remota en miles de dispositivos.
- **Optimización de Recursos:** Asegurar que el impacto en CPU/RAM se mantenga mínimo (<0.5% CPU) a pesar de la recolección constante de métricas.

## 5. Cumplimiento y Transparencia
- **Auditoría de Privacidad Detallada:** El "Privacy Hub" debe reflejar la integración con el GLPI, detallando qué activos están siendo monitoreados.
- **GDPR / Localización de Datos:** Asegurar que el flujo de datos ITAM cumpla con las normativas locales de privacidad.
- **EULA Dinámico:** Sistema de aceptación de términos legalmente vinculante integrado para el monitoreo de activos corporativos.

---

## Resumen de Esfuerzo Estimado

| Módulo | Esfuerzo | Impacto |
| :--- | :--- | :--- |
| Seguridad e Identidad | 2 Semanas | 🔥 Crítico |
| Integración GLPI + AI Alerts | 3 Semanas | 🛠️ Operativo |
| Modelos de Machine Learning | 5 Semanas | 🧠 Inteligencia |
| MQTT + Grafana Cloud | 3 Semanas | 📈 Visibilidad |
| Sensores Hardened (Rust) | 4 Semanas | ⚙️ Precisión |

---
*Documentación generada para la alineación estratégica con Zinko ITAM Predictivo.*
