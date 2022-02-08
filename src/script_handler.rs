use crate::config::Config;
use std::path::PathBuf;

pub fn create(temp_path: PathBuf, video_path: PathBuf, settings: Config) -> PathBuf {
    let script_filename = temp_path.join(rand::random::<u16>().to_string() + ".vpk");

    let mut script = "from vapoursynth import core\nimport vapoursynth as vs\nimport havsfunc as haf\nimport adjust\nimport weighting\n".to_owned();

    if settings.deduplicate {
        script += "import filldrops\n";
    }

    if settings.interpolation_program == "rife" {
        script += "from vsrife import RIFE\n";
    }

    let extentension = video_path.extension().unwrap().to_str().unwrap();
    if extentension != ".avi" {
        script += format!(
            "video = core.ffms2.Source(source=\"{}\")\n",
            video_path.to_str().unwrap().replace("\\", "\\\\")
        )
        .as_str();
    } else {
        script += format!(
            "video = core.avisource.AVISource(\"{}\")",
            video_path.to_str().unwrap()
        )
        .as_str();
        script += "video = core.fmtc.matrix(clip=video, mat=\"601\", col_fam=vs.YUV, bits=16)\n";
        script += "video = core.fmtc.resample(clip=video, css\"420\")\n";
        script += "video = core.fmtc.bitdepth(clip=video, bits=8)\n";
    }

    if settings.deduplicate {
        script += "video = filldrops.FillDrops(video, thresh=0.001)\n";
    }

    if settings.input_timescale != 1.0 {
        script += format!(
            "video = core.std.AssumeFPS(video, fpsnum=(video.fps * (1 / {})))",
            settings.input_timescale,
        )
        .as_str();
    }

    if settings.interpolate {
        if settings.interpolation_program == "rife" {
            script += "video = core.resize.Bicubic(video, format=vs.RGBS)\n";
            script += format!("while videofps < {}:\n", settings.interpolated_fps).as_str();
            script += "    video = RIFE(video)\n";
            script += "video = core.resize.Bicubic(video, format=vs.YUV402P8, matrix_x=\"709\")\n"
        } else if settings.interpolation_program == "rife-ncnn" {
            script += "video = core.resize.Bicubic(video, format=vs.RGBS)\n";

            script += format!("while videofps < {}:\n", settings.interpolated_fps).as_str();
            script += "    video = core.rife.RIFE(video)\n";

            script += "video = core.resize.Bicubic(video, format=vs.YUV402P8, matrix_x=\"709\")\n"
        } else {
            let mut speed = settings.interpolation_speed;
            if speed.to_lowercase() == "default" {
                speed = "medium".to_string();
            }

            let mut tuning = settings.interpolation_tuning;
            if tuning.to_lowercase() == "default" {
                tuning = "smooth".to_string();
            }

            let mut algorithm = settings.interpolation_algorithm;
            if algorithm.to_lowercase() == "default" {
                algorithm = "13".to_string();
            }

            let gpu_bool;
            if settings.gpu {
                gpu_bool = "True";
            } else {
                gpu_bool = "False";
            }
            script += format!("video = haf.InterFrame(video, GPU={}, NewNum={}, Preset=\"{}\", Tuning=\"{}\", OverrideAlgo={})\n", gpu_bool, settings.interpolated_fps, speed, tuning, algorithm).as_str()
        }
    }

    if settings.output_timescale != 1.0 {
        script += format!(
            "video = core.std.AssumeFPS(video, fpsnum=(video.fps * {}))\n",
            settings.output_timescale,
        )
        .as_str();
    }

    if settings.blur {
        script += format!(
            "frame_gap = int(video.fps / {})\n",
            settings.blur_output_fps
        )
        .as_str();
        script += format!(
            "blended_frames = int(frame_gap *{})\n",
            settings.blur_amount
        )
        .as_str();

        script += "if blended_frames > 0:\n";
        script += "    if blended_frames % 2 == 0:\n";
        script += "        blended_frames += 1\n";

        let triangle_reverse_bool;
        if settings.blur_weighting_triangle_reverse {
            triangle_reverse_bool = "True";
        } else {
            triangle_reverse_bool = "False";
        }

        script += "    video = core.frameblender.FrameBlend(video, weighting.equal(blended_frames), True)\n";

        script += format!(
            "video = haf.ChangeFPS(video, {})\n",
            settings.blur_output_fps
        )
        .as_str();
    }

    if settings.brightness != 1.0 || settings.contrast != 1.0 || settings.saturation != 1.0 {
        script += format!(
            "video = adjust.Tweak(video, bright={}, cont={}, sat={})\n",
            settings.brightness, settings.contrast, settings.saturation
        )
        .as_str();
    }

    script += "video.set_output()\n";
    let file = script_filename.clone();
    std::fs::write(script_filename, script).expect("Could not write script file");
    file
}
