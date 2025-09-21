#[macro_export]
macro_rules! define_resource_bundle {
    (
        $resource_name:ident,        // ex: BlockHandles
        $handle_name:ident,          // ex: BlockHandle
        $enum_name:ident,            // ex: BlockType
        $component_name:ident,       // ex: BlockComponent
        $bundle_name:ident,          // ex: BlockBundle
        {
            $(
                $variant:ident => {
                    mesh: $mesh:expr,
                    material: $material:expr
                }
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $enum_name {
            $($variant),*
        }

        #[derive(Debug, Component)]
        pub struct $component_name {
            pub inner_type: $enum_name,
        }

        #[derive(Debug, Resource, Clone)]
        pub struct $handle_name {
            pub mesh: Handle<Mesh>,
            pub material: Handle<StandardMaterial>,
            pub component: $enum_name,
        }

        #[derive(Debug, Resource, Clone)]
        pub struct $resource_name {
            $(pub $variant: $handle_name),*
        }


        pub fn load_resource(
            mut commands: Commands,
            mut meshes: ResMut<Assets<Mesh>>,
            mut materials: ResMut<Assets<StandardMaterial>>,
        ) {
            let resource = $resource_name {
                $(
                    $variant: $handle_name {
                        mesh: meshes.add($mesh),
                        material: materials.add($material),
                        component: $enum_name::$variant,
                    }
                ),*
            };

            commands.insert_resource(resource);
        }

        #[derive(Debug,Bundle)]
        pub struct $bundle_name {
            pub mesh: Mesh3d,
            pub material: MeshMaterial3d<StandardMaterial>,
            pub position: Transform,
            component: $component_name,
        }

        impl $bundle_name {
            pub fn new(handle: &$handle_name, position: Vec3) -> Self {
                Self {
                    mesh: Mesh3d(handle.mesh.clone()),
                    material: MeshMaterial3d(handle.material.clone()),
                    position: Transform::from_translation(position),
                    component: $component_name { inner_type: handle.component },
                }
            }
        }
    }
}
