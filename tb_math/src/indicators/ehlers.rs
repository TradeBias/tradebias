use std::f64::consts::PI;

pub fn ehlers_super_smoother(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < 3 || period < 1 { return out; }
    
    let a1 = ((-1.414 * PI) / period as f64).exp();
    let b1 = 2.0 * a1 * ((1.414 * PI) / period as f64).cos();
    let c2 = b1;
    let c3 = -a1 * a1;
    let c1 = 1.0 - c2 - c3;
    
    for i in 0..data.len() {
        if i < 2 { 
            out[i] = data[i]; 
            continue; 
        }
        let prev1 = if out[i-1].is_nan() { data[i] } else { out[i-1] };
        let prev2 = if out[i-2].is_nan() { data[i] } else { out[i-2] };
        out[i] = c1 * ((data[i] + data[i-1]) / 2.0) + c2 * prev1 + c3 * prev2;
    }
    out
}

pub fn ehlers_decycler(data: &[f64], period: usize) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < 3 { return out; }
    
    let angle = (2.0 * PI) / period as f64;
    let alpha = (angle.cos() + angle.sin() - 1.0) / angle.cos();
    
    let mut hp = vec![0.0; data.len()];
    
    for i in 0..data.len() {
        if i < 2 {
            hp[i] = 0.0;
            out[i] = data[i];
            continue;
        }
        hp[i] = (1.0 - alpha / 2.0).powi(2) * (data[i] - 2.0 * data[i-1] + data[i-2]) 
                + 2.0 * (1.0 - alpha) * hp[i-1] 
                - (1.0 - alpha).powi(2) * hp[i-2];
        out[i] = data[i] - hp[i];
    }
    out
}

pub fn ehlers_cyber_cycle(data: &[f64], alpha: f64) -> Vec<f64> {
    let mut out = vec![f64::NAN; data.len()];
    if data.len() < 4 { return out; }
    
    let mut smooth = vec![0.0; data.len()];
    for i in 3..data.len() {
        smooth[i] = (data[i] + 2.0 * data[i-1] + 2.0 * data[i-2] + data[i-3]) / 6.0;
    }
    
    let mut cycle = vec![0.0; data.len()];
    for i in 0..data.len() {
        if i < 4 {
            cycle[i] = data[i];
            continue;
        }
        cycle[i] = (1.0 - 0.5 * alpha).powi(2) * (smooth[i] - 2.0 * smooth[i-1] + smooth[i-2])
                 + 2.0 * (1.0 - alpha) * cycle[i-1]
                 - (1.0 - alpha).powi(2) * cycle[i-2];
    }
    cycle
}

pub struct MamaFama {
    pub mama: Vec<f64>,
    pub fama: Vec<f64>,
}

pub fn ehlers_mama_fama(data: &[f64], fast_limit: f64, slow_limit: f64) -> MamaFama {
    let n = data.len();
    let mut res = MamaFama {
        mama: vec![0.0; n],
        fama: vec![0.0; n],
    };
    if n < 7 { return res; }
    
    let mut smooth = vec![0.0; n];
    let mut detrender = vec![0.0; n];
    let mut q1 = vec![0.0; n];
    let mut i1 = vec![0.0; n];
    let mut j_i = vec![0.0; n];
    let mut j_q = vec![0.0; n];
    let mut i2 = vec![0.0; n];
    let mut q2 = vec![0.0; n];
    let mut re = vec![0.0; n];
    let mut im = vec![0.0; n];
    let mut phase = vec![0.0; n];
    let mut period = vec![0.0; n];

    for i in 0..n {
        if i < 6 {
            res.mama[i] = data[i];
            res.fama[i] = data[i];
            period[i] = 0.0;
            continue;
        }
        
        smooth[i] = (4.0 * data[i] + 3.0 * data[i-1] + 2.0 * data[i-2] + data[i-3]) / 10.0;
        let p = if period[i-1] != 0.0 { period[i-1] } else { 1.0 };
        let adj = 0.075 * p + 0.54;
        
        detrender[i] = (0.0962 * smooth[i] + 0.5769 * smooth[i-2] - 0.5769 * smooth[i-4] - 0.0962 * smooth[i-6]) * adj;
        
        q1[i] = (0.0962 * detrender[i] + 0.5769 * detrender[i-2] - 0.5769 * detrender[i-4] - 0.0962 * detrender[i-6]) * adj;
        i1[i] = detrender[i-3];
        
        j_i[i] = (0.0962 * i1[i] + 0.5769 * i1[i-2] - 0.5769 * i1[i-4] - 0.0962 * i1[i-6]) * adj;
        j_q[i] = (0.0962 * q1[i] + 0.5769 * q1[i-2] - 0.5769 * q1[i-4] - 0.0962 * q1[i-6]) * adj;
        
        i2[i] = i1[i] - j_q[i];
        q2[i] = q1[i] + j_i[i];
        
        i2[i] = 0.2 * i2[i] + 0.8 * i2[i-1];
        q2[i] = 0.2 * q2[i] + 0.8 * q2[i-1];
        
        re[i] = i2[i] * i2[i-1] + q2[i] * q2[i-1];
        im[i] = i2[i] * q2[i-1] - q2[i] * i2[i-1];
        
        re[i] = 0.2 * re[i] + 0.8 * re[i-1];
        im[i] = 0.2 * im[i] + 0.8 * im[i-1];
        
        if re[i] != 0.0 && im[i] != 0.0 {
            period[i] = 360.0 / (im[i] / re[i]).atan().to_degrees();
        } else {
            period[i] = period[i-1];
        }
        
        if period[i] > 1.5 * period[i-1] { period[i] = 1.5 * period[i-1]; }
        if period[i] < 0.67 * period[i-1] { period[i] = 0.67 * period[i-1]; }
        if period[i] < 6.0 { period[i] = 6.0; }
        if period[i] > 50.0 { period[i] = 50.0; }
        
        period[i] = 0.2 * period[i] + 0.8 * period[i-1];
        
        if i1[i] != 0.0 { phase[i] = (q1[i] / i1[i]).atan().to_degrees(); }
        
        let mut delta_phase = phase[i-1] - phase[i];
        if delta_phase < 1.0 { delta_phase = 1.0; }
        
        let mut alpha = fast_limit / delta_phase;
        if alpha < slow_limit { alpha = slow_limit; }
        if alpha > fast_limit { alpha = fast_limit; }
        
        res.mama[i] = alpha * data[i] + (1.0 - alpha) * res.mama[i-1];
        res.fama[i] = 0.5 * alpha * res.mama[i] + (1.0 - 0.5 * alpha) * res.fama[i-1];
    }
    res
}

pub fn ehlers_mama(data: &[f64], fast_limit: f64, slow_limit: f64) -> Vec<f64> { ehlers_mama_fama(data, fast_limit, slow_limit).mama }
pub fn ehlers_fama(data: &[f64], fast_limit: f64, slow_limit: f64) -> Vec<f64> { ehlers_mama_fama(data, fast_limit, slow_limit).fama }

pub struct SineWave {
    pub sine: Vec<f64>,
    pub lead_sine: Vec<f64>,
}

pub fn ehlers_sine_wave(data: &[f64]) -> SineWave {
    let n = data.len();
    let mut res = SineWave {
        sine: vec![0.0; n],
        lead_sine: vec![0.0; n],
    };
    if n < 7 { return res; }
    
    let mut smooth = vec![0.0; n];
    let mut detrender = vec![0.0; n];
    let mut q1 = vec![0.0; n];
    let mut i1 = vec![0.0; n];
    let mut j_i = vec![0.0; n];
    let mut j_q = vec![0.0; n];
    let mut i2 = vec![0.0; n];
    let mut q2 = vec![0.0; n];
    let mut re = vec![0.0; n];
    let mut im = vec![0.0; n];
    let mut period = vec![0.0; n];
    let mut smooth_period = vec![0.0; n];
    let mut phase = vec![0.0; n];

    for i in 0..n {
        if i < 6 {
            res.sine[i] = 0.0;
            res.lead_sine[i] = 0.0;
            period[i] = 0.0;
            continue;
        }
        
        smooth[i] = (4.0 * data[i] + 3.0 * data[i-1] + 2.0 * data[i-2] + data[i-3]) / 10.0;
        let p = if period[i-1] != 0.0 { period[i-1] } else { 1.0 };
        let adj = 0.075 * p + 0.54;
        
        detrender[i] = (0.0962 * smooth[i] + 0.5769 * smooth[i-2] - 0.5769 * smooth[i-4] - 0.0962 * smooth[i-6]) * adj;
        
        q1[i] = (0.0962 * detrender[i] + 0.5769 * detrender[i-2] - 0.5769 * detrender[i-4] - 0.0962 * detrender[i-6]) * adj;
        i1[i] = detrender[i-3];
        
        j_i[i] = (0.0962 * i1[i] + 0.5769 * i1[i-2] - 0.5769 * i1[i-4] - 0.0962 * i1[i-6]) * adj;
        j_q[i] = (0.0962 * q1[i] + 0.5769 * q1[i-2] - 0.5769 * q1[i-4] - 0.0962 * q1[i-6]) * adj;
        
        i2[i] = i1[i] - j_q[i];
        q2[i] = q1[i] + j_i[i];
        
        i2[i] = 0.2 * i2[i] + 0.8 * i2[i-1];
        q2[i] = 0.2 * q2[i] + 0.8 * q2[i-1];
        
        re[i] = i2[i] * i2[i-1] + q2[i] * q2[i-1];
        im[i] = i2[i] * q2[i-1] - q2[i] * i2[i-1];
        
        re[i] = 0.2 * re[i] + 0.8 * re[i-1];
        im[i] = 0.2 * im[i] + 0.8 * im[i-1];
        
        if re[i] != 0.0 && im[i] != 0.0 {
            period[i] = 360.0 / (im[i] / re[i]).atan().to_degrees();
        } else {
            period[i] = period[i-1];
        }
        
        if period[i] > 1.5 * period[i-1] { period[i] = 1.5 * period[i-1]; }
        if period[i] < 0.67 * period[i-1] { period[i] = 0.67 * period[i-1]; }
        if period[i] < 6.0 { period[i] = 6.0; }
        if period[i] > 50.0 { period[i] = 50.0; }
        
        period[i] = 0.2 * period[i] + 0.8 * period[i-1];
        smooth_period[i] = 0.33 * period[i] + 0.67 * smooth_period[i-1];
        
        let mut dc_phase = 0.0;
        if i1[i] != 0.0 { dc_phase = (q1[i] / i1[i]).atan().to_degrees(); }
        
        let mut dc_phase_adj = dc_phase;
        if i1[i] < 0.0 { dc_phase_adj += 180.0; }
        if i1[i] > 0.0 && q1[i] < 0.0 { dc_phase_adj += 360.0; }
        
        phase[i] = dc_phase_adj;
        
        res.sine[i] = (phase[i].to_radians()).sin();
        res.lead_sine[i] = ((phase[i] + 45.0).to_radians()).sin();
    }
    res
}

pub fn ehlers_sine(data: &[f64]) -> Vec<f64> { ehlers_sine_wave(data).sine }
pub fn ehlers_lead_sine(data: &[f64]) -> Vec<f64> { ehlers_sine_wave(data).lead_sine }

pub fn ehlers_decycler_oscillator(data: &[f64], hp_period1: usize, hp_period2: usize) -> Vec<f64> {
    let d1 = ehlers_decycler(data, hp_period1);
    let d2 = ehlers_decycler(data, hp_period2);
    let mut out = vec![f64::NAN; data.len()];
    for i in 0..data.len() {
        out[i] = d1[i] - d2[i];
    }
    out
}

pub fn ehlers_roofing_filter(data: &[f64], hp_period: usize, lp_period: usize) -> Vec<f64> {
    let decycler = ehlers_decycler(data, hp_period);
    ehlers_super_smoother(&decycler, lp_period)
}

pub fn ehlers_dominant_cycle_period(data: &[f64]) -> Vec<f64> {
    let n = data.len();
    let mut res = vec![f64::NAN; n];
    if n < 7 { return res; }
    
    let mut smooth = vec![0.0; n];
    let mut detrender = vec![0.0; n];
    let mut q1 = vec![0.0; n];
    let mut i1 = vec![0.0; n];
    let mut j_i = vec![0.0; n];
    let mut j_q = vec![0.0; n];
    let mut i2 = vec![0.0; n];
    let mut q2 = vec![0.0; n];
    let mut re = vec![0.0; n];
    let mut im = vec![0.0; n];
    let mut period = vec![0.0; n];

    for i in 0..n {
        if i < 6 {
            period[i] = 0.0;
            res[i] = 0.0;
            continue;
        }
        
        smooth[i] = (4.0 * data[i] + 3.0 * data[i-1] + 2.0 * data[i-2] + data[i-3]) / 10.0;
        let p = if period[i-1] != 0.0 { period[i-1] } else { 1.0 };
        let adj = 0.075 * p + 0.54;
        
        detrender[i] = (0.0962 * smooth[i] + 0.5769 * smooth[i-2] - 0.5769 * smooth[i-4] - 0.0962 * smooth[i-6]) * adj;
        
        q1[i] = (0.0962 * detrender[i] + 0.5769 * detrender[i-2] - 0.5769 * detrender[i-4] - 0.0962 * detrender[i-6]) * adj;
        i1[i] = detrender[i-3];
        
        j_i[i] = (0.0962 * i1[i] + 0.5769 * i1[i-2] - 0.5769 * i1[i-4] - 0.0962 * i1[i-6]) * adj;
        j_q[i] = (0.0962 * q1[i] + 0.5769 * q1[i-2] - 0.5769 * q1[i-4] - 0.0962 * q1[i-6]) * adj;
        
        i2[i] = i1[i] - j_q[i];
        q2[i] = q1[i] + j_i[i];
        
        i2[i] = 0.2 * i2[i] + 0.8 * i2[i-1];
        q2[i] = 0.2 * q2[i] + 0.8 * q2[i-1];
        
        re[i] = i2[i] * i2[i-1] + q2[i] * q2[i-1];
        im[i] = i2[i] * q2[i-1] - q2[i] * i2[i-1];
        
        re[i] = 0.2 * re[i] + 0.8 * re[i-1];
        im[i] = 0.2 * im[i] + 0.8 * im[i-1];
        
        if re[i] != 0.0 && im[i] != 0.0 {
            period[i] = 360.0 / (im[i] / re[i]).atan().to_degrees();
        } else {
            period[i] = period[i-1];
        }
        
        if period[i] > 1.5 * period[i-1] { period[i] = 1.5 * period[i-1]; }
        if period[i] < 0.67 * period[i-1] { period[i] = 0.67 * period[i-1]; }
        if period[i] < 6.0 { period[i] = 6.0; }
        if period[i] > 50.0 { period[i] = 50.0; }
        
        period[i] = 0.2 * period[i] + 0.8 * period[i-1];
        res[i] = period[i];
    }
    res
}

pub fn ehlers_emd(data: &[f64], period: usize, fraction: f64) -> Vec<f64> {
    ehlers_decycler_oscillator(data, period, (period as f64 * (1.0 + fraction)) as usize)
}
