
use crate::core::*;
use crate::objects::*;
use crate::phong::*;

pub struct PhongDeferredInstancedMesh {
    context: Context,
    pub name: String,
    mesh: InstancedMesh,
    pub material: PhongMaterial
}

impl PhongDeferredInstancedMesh
{
    pub fn new(context: &Context, transformations: &[Mat4], cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        if cpu_mesh.normals.is_none() {
            Err(Error::FailedToCreateMesh {message:
              "Cannot create a mesh without normals. Consider calling compute_normals on the CPUMesh before creating the mesh.".to_string()})?
        }
        let mesh = InstancedMesh::new(context, transformations, cpu_mesh)?;
        unsafe {INSTANCED_MESH_COUNT += 1;}
        Ok(Self {
            context: context.clone(),
            name: cpu_mesh.name.clone(),
            mesh,
            material: material.clone()
        })
    }

    pub fn update_transformations(&mut self, transformations: &[Mat4])
    {
        self.mesh.update_transformations(transformations);
    }

    pub fn render_depth(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        self.render_geometry(render_states, viewport, transformation, camera)
    }

    pub fn render_geometry(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => {
                unsafe {
                    if INSTANCED_PROGRAM_COLOR.is_none()
                    {
                        INSTANCED_PROGRAM_COLOR = Some(InstancedMesh::create_program(&self.context, &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/colored_deferred.frag")))?);
                    }
                    INSTANCED_PROGRAM_COLOR.as_ref().unwrap()
                }
            },
            ColorSource::Texture(_) => {
                unsafe {
                    if INSTANCED_PROGRAM_TEXTURE.is_none()
                    {
                        INSTANCED_PROGRAM_TEXTURE = Some(InstancedMesh::create_program(&self.context, &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/textured_deferred.frag")))?);
                    }
                    INSTANCED_PROGRAM_TEXTURE.as_ref().unwrap()
                }
            }
        };
        self.material.bind(program)?;
        self.mesh.render(program, render_states, viewport, transformation, camera)
    }
}

impl Drop for PhongDeferredInstancedMesh {

    fn drop(&mut self) {
        unsafe {
            INSTANCED_MESH_COUNT -= 1;
            if INSTANCED_MESH_COUNT == 0 {
                INSTANCED_PROGRAM_COLOR = None;
                INSTANCED_PROGRAM_TEXTURE = None;
            }
        }
    }
}

static mut INSTANCED_PROGRAM_COLOR: Option<Program> = None;
static mut INSTANCED_PROGRAM_TEXTURE: Option<Program> = None;
static mut INSTANCED_MESH_COUNT: u32 = 0;