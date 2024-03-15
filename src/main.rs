use std::{fs, path::PathBuf};

use cli::{
    cli,
    pipeline::{decode, encoder, operations},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use zune_core::colorspace::ColorSpace;
use zune_image::{
    core_filters::colorspace::ColorspaceConv, image::Image, metadata::AlphaState,
    pipelines::Pipeline,
};
use zune_imageprocs::premul_alpha::PremultiplyAlpha;

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

    let files = matches
        .get_many::<PathBuf>("files")
        .expect("`files` is required")
        .collect::<Vec<_>>();

    files.par_iter().for_each(|f| {
        let mut pipeline = Pipeline::<Image>::new();

        let img = match decode(f) {
            Ok(img) => img,
            Err(e) => {
                log::error!("{f:?}: {e}");
                return;
            }
        };

        let (encoder, ext) = match encoder(&matches) {
            Ok(encoder) => encoder,
            Err(e) => {
                log::error!("{f:?}: {e}");
                return;
            }
        };

        operations(&matches, &img)
            .into_iter()
            .for_each(|(_, operations)| match operations.name() {
                "fast resize" => {
                    pipeline
                        .add_operation(Box::new(PremultiplyAlpha::new(AlphaState::PreMultiplied)));

                    pipeline.add_operation(operations);

                    pipeline.add_operation(Box::new(PremultiplyAlpha::new(
                        AlphaState::NonPreMultiplied,
                    )));
                }
                "quantize" => {
                    pipeline.add_operation(Box::new(ColorspaceConv::new(ColorSpace::RGBA)));
                    pipeline.add_operation(operations);
                }
                _ => {
                    pipeline.add_operation(operations);
                }
            });

        pipeline.add_decoder(img);
        pipeline.add_encoder(encoder);

        match pipeline.advance_to_end() {
            Ok(()) => {}
            Err(e) => {
                log::error!("{f:?}: {e}");
                return;
            }
        };

        match fs::write(format!("image.{ext}"), pipeline.get_results()[0].data()) {
            Ok(()) => {}
            Err(e) => {
                log::error!("{f:?}: {e}");
            }
        };
    });
}
