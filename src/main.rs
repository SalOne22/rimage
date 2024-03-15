use cli::{cli, preprocessors::pipeline::PreprocessorPipeline};
use zune_image::{image::Image, pipelines::Pipeline};

mod cli;

fn main() {
    pretty_env_logger::init();

    let matches = cli().get_matches();

    if let Some(threads) = matches.get_one::<u8>("threads") {
        rayon::ThreadPoolBuilder::new()
            .num_threads(*threads as usize)
            .build_global()
            .unwrap();
    }

    let mut pipeline = Pipeline::<Image>::new();

    let preprocessor_pipeline = PreprocessorPipeline::from_matches(&matches);
}
