use audrey::read::Reader;
use std::fs::File;
use std::io;
use std::io::BufReader;
use rustfft::{FftPlanner, num_complex::Complex};
use std::path::Path;
use cute::c;


fn readFile(filePath:&Path) -> File{
    let mut f = File::open(filePath).unwrap();
    f
}


fn parseReader(audioSample: File, fileType: &str) -> (audrey::Reader<BufReader<File>>, u32){

    let reader = io::BufReader::new(audioSample);


    if (fileType == "flac"){
        let (audreyReader, channels) = match Reader::new(reader) {
            Ok(s) => {
                let sReader = s;
                let description = &sReader.description();
                let channels = description.channel_count();
                (sReader, channels)
            },
            Err(e) => panic!("{}", e) 
        };
        return (audreyReader, channels);
    }
    else if (fileType == "mp3"){
        return ()
    }

    return (audreyReader, channels);
}


fn mp3SampleArray(){

}

fn audreySampleArray(){

}



fn convertAudioToComplex(mut reader: Reader<BufReader<File>>, channels: u32) -> Vec<Complex<f64>>{
    println!("{:?}", reader.description());

    let mut sampleArray = Vec::<f64>::new();

    let samples = reader.samples::<f64>();

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
    let filePath = Path::new("C:/Users/ashka/Documents/Coding/Guitar_Tab_Generator/A2_mp3.Flac");     
    let fileType = filePath.extension().unwrap().to_str().unwrap();

    let file = readFile(filePath);
    
    let (audioSample, channels) = parseReader(file, fileType);

    let complexSamples = convertAudioToComplex(audioSample, channels);
    
    let complexSampleLength = &complexSamples.len();

    let fftTransform = performFFT(complexSamples);

    let magnitudeArray = c![calculateMagnitude(&x), for x in fftTransform];

    //what the fuck is this next function...it's the find the max value of the array WHY CAN'T I JUST DO magnitudeArray.max?!?!?!?
    let maxMagnitude = magnitudeArray.iter().cloned().fold(0./0., f64::max);
    println!("{}", maxMagnitude);

    let mut maxMagnitudeIter = magnitudeArray.iter();

    let maxMagnitudeIndex = maxMagnitudeIter.position(|&x| x == maxMagnitude).unwrap();

    let maxFrequency = (maxMagnitudeIndex) as f64 * 48000.0 / *(complexSampleLength) as f64;

    println!("{}", maxFrequency);
}
