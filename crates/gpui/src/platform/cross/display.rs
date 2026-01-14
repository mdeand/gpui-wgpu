use crate::{Bounds, DisplayId, Pixels, PlatformDisplay, PlatformWindow, Point, Size};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct WinitDisplay {
    display_id: DisplayId,
    handle: winit::monitor::MonitorHandle,
}

impl PlatformDisplay for WinitDisplay {
    fn id(&self) -> DisplayId {
        self.display_id
    }

    fn uuid(&self) -> anyhow::Result<uuid::Uuid> {
        let name = self
            .handle
            .name()
            .ok_or_else(|| anyhow::anyhow!("Failed to get monitor name"))?;

        Ok(Uuid::new_v5(&Uuid::NAMESPACE_DNS, name.as_bytes()))
    }

    fn bounds(&self) -> Bounds<Pixels> {
        // TODO(mdeand): Double check that this is correct?

        let size: winit::dpi::LogicalSize<f32> =
            self.handle.size().to_logical(self.handle.scale_factor());

        let origin: winit::dpi::LogicalPosition<f32> = self
            .handle
            .position()
            .to_logical(self.handle.scale_factor());

        Bounds {
            origin: Point {
                x: Pixels(origin.x as f32),
                y: Pixels(origin.y as f32),
            },
            size: Size {
                width: Pixels(size.width as f32),
                height: Pixels(size.height as f32),
            },
        }
    }
}
