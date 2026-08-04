fn yin_pitch(samples: &[f32], sample_rate: f32) -> Option<f32>{
    let n = samples.len() / 2;
    let mut diff = vec![0.0f32; n];

    //step 1. difference function
    for tau in 1..n { 
        let mut sum = 0.0;
        for i in 0..n {
            let d = samples[i] - samples[i+tau]; //keep multiplying data samples to find autocorrelation
            sum += d * d;
        }
        diff[tau] = sum;
    }
    //step 2. normalize the difference 
    let mut cmnd = vec![1.0f32; n];
    let mut running_sum = 0.0;
    for tau in 1..n {
        running_sum += diff[tau];
        cmnd[tau] = diff[tau] * tau as f32 / running_sum; 
    }

    //step 3. absolute thresholding
    let threshold = 0.1;
    let mut tau_estimate = None;
    for tau in 2..n {
        if cmnd[tau] < threshold { //the first local min/max that we find
            tau_estimate = Some(cmnd[tau]);
            break;
        }
    }
    //step 4. convert to Hz 
    tau_estimate.map(|tau| sample_rate / tau as f32)
}
