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


// fn parseReader(audioSample: File, fileType: &str) -> (audrey::Reader<BufReader<File>>, u32){

    
//     else if (fileType == "mp3"){
//         return ()
//     }

//     return (audreyReader, channels);
// }


fn mp3SampleArray(){

}

fn audreySampleArray(){

}

fn convertMP3ToComplex(audioSample: File){
    
    let mut reader = Decoder::new(audioSample);

    println!("Channels: {}, Sample rate: {}", &reader.next_frame().unwrap().channels, &reader.next_frame().unwrap().sample_rate);
    
    
    
    
    // let mut sampleArray = Vec::<f64>::new();

    // let samples = audreyReader.samples::<f64>();

    // for sample in samples {
    //     sampleArray.push(sample.expect("Error at Pushing Sample Array"))
    // } 

    // let complexSamples: Vec<Complex<f64>> = c![Complex::new(x,0.0), for x in sampleArray];   

    // return complexSamples;
    return;
}


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

    let complexSamples: Vec<Complex<f64>> = c![Complex::new(x,0.0), for x in sampleArray];   

    return complexSamples;
}


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
    let filePath = Path::new("/users/aramesh/Downloads/E.mp3");     
    let fileType = filePath.extension().unwrap().to_str().unwrap();

    let file = readFile(filePath);
    
    if (fileType != "mp3") {
        convertNonMP3ToComplex(file);
    }
    else {
        convertMP3ToComplex(file);
    }

    
    
    // let complexSampleLength = &complexSamples.len();

    // let fftTransform = performFFT(complexSamples);

    // let magnitudeArray = c![calculateMagnitude(&x), for x in fftTransform];

    // //what the fuck is this next function...it's the find the max value of the array WHY CAN'T I JUST DO magnitudeArray.max?!?!?!?
    // let maxMagnitude = magnitudeArray.iter().cloned().fold(0./0., f64::max);
    // println!("{}", maxMagnitude);

    // let mut maxMagnitudeIter = magnitudeArray.iter();

    // let maxMagnitudeIndex = maxMagnitudeIter.position(|&x| x == maxMagnitude).unwrap();

    // let maxFrequency = (maxMagnitudeIndex) as f64 * 48000.0 / *(complexSampleLength) as f64;

    // println!("{}", maxFrequency);
}
