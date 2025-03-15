# Centro de Actualización de GNOME

El Centro de Actualización de GNOME proporciona una interfaz unificada para gestionar todos los tipos de actualizaciones del sistema. Combina:

- Actualizaciones de software (anteriormente "Actualizador de Software" / "Gestor de Actualizaciones")
- Actualizaciones de firmware (anteriormente "Actualizador de Firmware")
- Controladores de hardware (anteriormente "Controladores Adicionales")
- Gestión de repositorios (anteriormente parte de "Software y Actualizaciones")

## Características

- Interfaz moderna y limpia basada en GTK4 y libadwaita
- Sigue las Directrices de Interfaz Humana (HIG) de GNOME
- Panel unificado para todos los tipos de actualizaciones
- Compatible con múltiples distribuciones de Linux
- Soporte para PackageKit, fwupd y AppStream
- Opciones de configuración granular de actualizaciones

## Instalación

### Compilación desde el código fuente

#### Prerrequisitos

- Rust 1.69 o superior
- Bibliotecas de desarrollo de GTK4 y libadwaita
- Sistema de compilación Meson y Ninja

#### Pasos de compilación

```bash
# Clonar el repositorio
git clone https://gitlab.gnome.org/gnome/gnome-update-center.git
cd gnome-update-center

# Compilar con Cargo
cargo build --release

# Alternativamente, compilar con Meson
meson setup _build
ninja -C _build
sudo ninja -C _build install
```

### Paquetes de distribución

El Centro de Actualización de GNOME está disponible en los siguientes formatos:

- Flatpak: `flatpak install flathub org.gnome.UpdateCenter`
- Debian/Ubuntu: `sudo apt install gnome-update-center`
- Fedora: `sudo dnf install gnome-update-center`

## Uso

Inicia la aplicación desde el menú de aplicaciones o ejecuta:

```bash
gnome-update-center
```

### Integración con el sistema

La aplicación requiere ciertos permisos del sistema para funcionar correctamente:

- Acceso a PackageKit para actualizaciones de software
- Acceso a fwupd para actualizaciones de firmware
- PolicyKit (polkit) para operaciones con privilegios

Cuando se ejecuta como Flatpak, los permisos adecuados se solicitarán automáticamente.

## Desarrollo

Consulta el archivo `CONTRIBUTING.md` para conocer las directrices de desarrollo.

### Estructura del proyecto

- `src/`: Código fuente en Rust
  - `application.rs`: Aplicación principal
  - `config/`: Gestión de configuración
  - `ui/`: Componentes de la interfaz de usuario
  - `services/`: Servicios de backend
- `data/`: Recursos y definiciones de la interfaz de usuario
- `build-aux/`: Scripts de compilación y utilidades

## Licencia

Este proyecto está licenciado bajo la Licencia Pública General de GNU v3.0. Consulta el archivo LICENSE para más detalles.