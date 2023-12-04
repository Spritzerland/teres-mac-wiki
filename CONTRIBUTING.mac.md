## Install dev environment or manual install

### Requirements
- [Python](https://www.python.org/downloads)
- [FFmpeg](https://ffmpeg.org/download.html)
- [VapourSynth x64](https://www.vapoursynth.com)

### VapourSynth plugins
- [FFMS2](https://github.com/FFMS/ffms2)
- [HAvsFunc](https://github.com/HomeOfVapourSynthEvolution/havsfunc)
- [SVPflow v4.2.0.142](https://github.com/bjaan/smoothvideo/blob/main/SVPflow_LastGoodVersions.7z)
- [vs-frameblender](https://github.com/f0e/vs-frameblender)
- [weighting.py](https://github.com/f0e/blur/blob/master/plugins/weighting.py)
- [filldrops.py](https://github.com/f0e/blur/blob/master/plugins/filldrops.py)

1. Build teres with ``cargo``.
2. Install Python 3.10.12 with [pyenv](https://github.com/pyenv/pyenv)
3. Set Python 3.10.12 as the global Python with ``pyenv global 3.10.12``
4. Install FFmpeg with ``brew install ffmpeg``
5. Install VapourSynth with the command ``brew install vapoursynth``
6. Install the required VapourSynth plugins from [here](https://github.com/Spritzerland/svpflow-arm64) and place all of them in /opt/homebrew/lib/vapoursynth
7. Install mvsfunc with ``pip3 install git+https://github.com/HomeOfVapourSynthEvolution/mvsfunc``
8. Install HAvsFunc with ``pip install havsfunc``
9. Install [weighting.py](https://raw.githubusercontent.com/f0e/blur/master/plugins/weighting.py) and [filldrops.py](https://github.com/animafps/teres/blob/main/plugins/filldrops.py) to /.pyenv/versions/3.10.12/lib/python3.10/site-packages/
10. Add this to your ~/.zshrc/, replacing the path exported in PYTHONPATH as the absolute path to your /site-packages/ folder:
```
unset PYTHONPATH
export PYTHONPATH="{path/to/.pyenv/versions/3.10.12/lib/python3.10/site-packages/}"
```


### Documentation Environment

See [jekyll docs](https://jekyllrb.com/docs/)

## Guidelines for code

- Format with `cargo fmt`
- Follow all guidelines of https://clig.dev
