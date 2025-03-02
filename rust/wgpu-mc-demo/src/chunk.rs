use std::fmt::Debug;
use std::sync::Arc;
use std::time::Instant;

use wgpu_mc::mc::block::{BlockstateKey, ChunkBlockState};
use wgpu_mc::mc::chunk::{BlockStateProvider, Chunk};
use wgpu_mc::mc::MinecraftState;
use wgpu_mc::minecraft_assets::schemas::blockstates::multipart::StateValue;

use wgpu_mc::render::pipeline::BLOCK_ATLAS;
use wgpu_mc::WmRenderer;

struct SimpleBlockstateProvider(Arc<MinecraftState>, BlockstateKey);

impl BlockStateProvider for SimpleBlockstateProvider {
    fn get_state(&self, _x: i32, _y: i16, _z: i32) -> ChunkBlockState {
        ChunkBlockState::State(self.1)
    }

    fn is_section_empty(&self, _index: usize) -> bool {
        false
    }
}

impl Debug for SimpleBlockstateProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}

pub fn make_chunks(wm: &WmRenderer) -> Chunk {
    let bm = wm.mc.block_manager.read();
    let atlas = wm
        .mc
        .texture_manager
        .atlases
        .load()
        .get(BLOCK_ATLAS)
        .unwrap()
        .load();

    let (index, _, anvil) = bm.blocks.get_full("minecraft:anvil").unwrap();

    let (_, augment) = anvil
        .get_model_by_key(
            [("facing", &StateValue::String("north".into()))],
            &*wm.mc.resource_provider,
            &atlas,
        )
        .unwrap();

    let provider = SimpleBlockstateProvider(
        wm.mc.clone(),
        BlockstateKey {
            block: index as u16,
            augment,
        },
    );

    let chunk = Chunk::new([0, 0]);
    let time = Instant::now();

    let pipelines = wm.pipelines.load();
    let layers = pipelines.chunk_layers.load();

    chunk.bake_chunk(wm, &layers, &bm, &provider);

    println!(
        "Built 1 chunk in {} microseconds",
        Instant::now().duration_since(time).as_micros()
    );

    chunk
}
