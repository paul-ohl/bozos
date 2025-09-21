use bevy::{log::tracing, prelude::*};

#[derive(Debug, Component)]
pub struct BlockComponent {
    pub block_type: BlockType,
}

#[derive(Debug, Resource, Clone)]
pub struct BlockHandle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub block_type: BlockType,
}

#[derive(Debug, Clone, Copy)]
pub enum BlockType {
    Dirt,
    Grass,
    Stone,
    Bedrock,
}

#[derive(Debug, Resource, Clone)]
pub struct BlockHandles {
    pub dirt_handle: BlockHandle,
    pub grass_handle: BlockHandle,
    pub stone_handle: BlockHandle,
    pub bedrock_handle: BlockHandle,
}

#[tracing::instrument(skip(commands, meshes, materials))]
pub fn load_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let block_handles = BlockHandles {
        dirt_handle: BlockHandle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(134, 96, 67)),
            block_type: BlockType::Dirt,
        },
        grass_handle: BlockHandle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(97, 159, 62)),
            block_type: BlockType::Grass,
        },
        stone_handle: BlockHandle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(125, 125, 125)),
            block_type: BlockType::Stone,
        },
        bedrock_handle: BlockHandle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(54, 54, 54)),
            block_type: BlockType::Bedrock,
        },
    };

    commands.insert_resource(block_handles);
}

#[derive(Bundle)]
pub struct BlockBundle {
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub position: Transform,
    block_component: BlockComponent,
}

impl BlockBundle {
    pub fn new(block_type: BlockType, position: Vec3, block_handle: &BlockHandle) -> Self {
        Self {
            mesh: Mesh3d(block_handle.mesh.clone()),
            material: MeshMaterial3d(block_handle.material.clone()),
            position: Transform::from_translation(position),
            block_component: BlockComponent { block_type },
        }
    }
}
