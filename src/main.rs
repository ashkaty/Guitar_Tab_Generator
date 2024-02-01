use audrey::read::Reader;
use std::fs::File;
use std::io;
use std::io::BufReader;
use rustfft::{FftPlanner, num_complex::Complex};
use std::path::Path;
use minimp3::{Decoder, Frame, Error};
use cute::c;


fn readFile(filePath:&Path) -> File{
    let mut f = File::open(filePath).unwrap();
    f
}

//description: This file takes in a sample array and edits it so the data is representative of a mono signal
fn stereoToMono(sampleArray: Vec<f64>) -> Vec<f64>{
    let sampleArrayChunks = sampleArray.chunks(2);
    let monoSampleArray = sampleArrayChunks.map(|chunk| (chunk[0] + chunk[1]) / 2.0).collect::<Vec<f64>>();
    return monoSampleArray
}

//description: this function reades each audio sample only for mp3 files and converts it to an Vector of complex numbers where the real number is the data in each sample and the imaginary number is 0
//reason: In order to perform the FFT algorithm to parse apart analog waves into frequencies, the FFT algo takes in complex numbers. The logic behind getting the values of each sample for mp3 files is different because of a different crate 
fn convertMP3ToComplex(audioSample: File) -> Vec<Complex<f64>>{
    
    let mut reader = Decoder::new(audioSample);

    println!("Format: MP3, Channels: {}, Sample rate: {}", &reader.next_frame().unwrap().channels, &reader.next_frame().unwrap().sample_rate);
    

    
    let mut sampleArray = Vec::<f64>::new();

    

    loop {
        match reader.next_frame() {
            Ok(Frame {data, ..}) => {
                for sample in data {
                    sampleArray.push(sample.into())
                };
            },
            Err(Error::Eof) => break,
            Err(e) => panic!("{}", e),
        }
    }

    let monoSampleArray = stereoToMono(sampleArray);

    let complexSamples: Vec<Complex<f64>> = c![Complex::new(x,0.0), for x in monoSampleArray];   


    return complexSamples;
}

//description: this function reades each audio sample only for mp3 files and converts it to an Vector of complex numbers where the real number is the data in each sample and the imaginary number is 0
//reason: In order to perform the FFT algorithm to parse apart analog waves into frequencies, the FFT algo takes in complex numbers. The logic behind getting the values of each sample for mp3 files is different because of a different crate 
fn convertNonMP3ToComplex(audioSample: File) -> Vec<Complex<f64>>{
    
    let reader = io::BufReader::new(audioSample);
    
    
    let (mut audreyReader, channels) = match Reader::new(reader) {
        Ok(s) => {
            let sReader = s;
            let description = &sReader.description();
            let channels = description.channel_count();
            (sReader, channels)
        },
        Err(e) => panic!("{}", e) 
    };
    
    println!("{:?}", &audreyReader.description());
    
    
    let mut sampleArray = Vec::<f64>::new();

    let samples = audreyReader.samples::<f64>();

    for sample in samples {
        sampleArray.push(sample.expect("Error at Pushing Sample Array"))
    } 

    let monoSampleArray = stereoToMono(sampleArray);
    
    let complexSamples: Vec<Complex<f64>> = c![Complex::new(x,0.0), for x in monoSampleArray];   

    return complexSamples;
}

//description: this function performs an fft algorithm on the set of sample data and returns the same sample array with each number changed
//reason: the fft algorithm basically creates a vector where the index represents the frequency and the real number part of the value represents amplitude (or how loud that frequency is)
fn performFFT(mut complexArray: Vec<Complex<f64>>) -> Vec<Complex<f64>>{
    let mut planner:FftPlanner<f64> = FftPlanner::new();
    let fft = planner.plan_fft_forward(complexArray.len());
    fft.process(&mut complexArray);
    return complexArray;
}

fn calculateMagnitude(complex: &Complex<f64>) -> f64 {
    (complex.re.powi(2) + complex.im.powi(2)).sqrt()
}


fn main() {
    let filePath = Path::new("C:/Users/ashka/Documents/Coding/Guitar_Tab_Generator/A2_Flac.mp3");     
    let fileType = filePath.extension().unwrap().to_str().unwrap();

    let file = readFile(filePath);
    
    let complexSamples:Vec<Complex<f64>>;

    if (fileType != "mp3") {
        complexSamples = convertNonMP3ToComplex(file);
    }
    else {
        complexSamples = convertMP3ToComplex(file);
    }

    
    
    let complexSampleLength = &complexSamples.len();

    let fftTransform = performFFT(complexSamples);

    let magnitudeArray = c![calculateMagnitude(&x), for x in fftTransform];

    //what the fuck is this next function...it's the find the max value of the array WHY CAN'T I JUST DO magnitudeArray.max?!?!?!?
    let maxMagnitude = magnitudeArray.iter().cloned().fold(0./0., f64::max);

    let mut maxMagnitudeIter = magnitudeArray.iter();

    let maxMagnitudeIndex = maxMagnitudeIter.position(|&x| x == maxMagnitude).unwrap();

  
    let maxFrequency = ((maxMagnitudeIndex) as f64 * 48000.0 / *(complexSampleLength) as f64);

    println!("{}", maxFrequency);
}