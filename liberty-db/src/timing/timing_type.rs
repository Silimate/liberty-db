//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html');
//! </script>
use std::{fmt::Display, str::FromStr};

use crate::{expression::{EdgeState, StaticState}, types::MaxMin};


/// # Combinational Timing Arcs
/// 
/// The timing type and timing sense define the signal propagation pattern. 
/// The default timing type is combinational. Table shows the timing 
/// type and timing sense values for combinational timing arcs.
/// The default timing type is combinational.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =331.4
/// &end
/// =331.8
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html');
/// </script>
/// 
/// <table>
///     <thead>
///         <tr>
///             <th>Timing type</th>
///             <th></th>
///             <th>Timing sense</th>
///             <th></th>
///         </tr>
///         <tr>
///             <th></th>
///             <th><code>positive_unate</code></th>
///             <th><code>negative_unate</code></th>
///             <th><code>non_unate</code></th>
///         </tr>
///     </thead>
///     <tbody>
///         <tr>
///             <td><code>combinational</code></td>
///             <td>R->R, F->F</td>
///             <td>R->F, F->R</td>
///             <td>{R,F}->{R,F}</td>
///         </tr>
///         <tr>
///             <td><code>combinational_rise</code></td>
///             <td>R->R</td>
///             <td>F->R</td>
///             <td>{R,F}->R</td>
///         </tr>
///         <tr>
///             <td><code>combinational_fall</code></td>
///             <td>F->F</td>
///             <td>R->F</td>
///             <td>{R,F}->F</td>
///         </tr>
///         <tr>
///             <td><code>three_state_disable</code></td>
///             <td>R->{0Z,1Z}</td>
///             <td>F->{0Z,1Z}</td>
///             <td>{R,F}->{0Z,1Z}</td>
///         </tr>
///         <tr>
///             <td><code>three_state_enable</code></td>
///             <td>R->{Z0,Z1}</td>
///             <td>F->{Z0,Z1}</td>
///             <td>{R,F}->{Z0,Z1}</td>
///         </tr>
///         <tr>
///             <td><code>three_state_disable_rise</code></td>
///             <td>R->0Z</td>
///             <td>F->0Z</td>
///             <td>{R,F}->0Z</td>
///         </tr>
///         <tr>
///             <td><code>three_state_disable_fall</code></td>
///             <td>R->1Z</td>
///             <td>F->1Z</td>
///             <td>{R,F}->1Z</td>
///         </tr>
///         <tr>
///             <td><code>three_state_enable_rise</code></td>
///             <td>R->Z1</td>
///             <td>F->Z1</td>
///             <td>{R,F}->Z1</td>
///         </tr>
///         <tr>
///             <td><code>three_state_enable_fall</code></td>
///             <td>R->Z0</td>
///             <td>F->Z0</td>
///             <td>{R,F}->Z0</td>
///         </tr>
///     </tbody>
/// </table>
#[derive(Debug, Clone, Copy)]
pub enum ArcCombinational{
    /// `combinational`(`Defualt`)/
    /// `combinational_rise`/
    /// `combinational_fall`
    /// 
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =331.11
    /// &end
    /// =331.13
    /// ">Reference</a>
    Combinational(Option<EdgeState>),
    /// `three_state_disable`/
    /// `three_state_disable_rise`/ 
    /// `three_state_disable_fall`
    /// 
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =331.14
    /// +331.16
    /// &end
    /// =331.14
    /// +331.17
    /// ">Reference</a>
    ThreeStateDisable(Option<EdgeState>),
    /// `three_state_enable`/
    /// `three_state_enable_rise`/
    /// `three_state_enable_fall`
    /// 
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =331.15
    /// +331.18
    /// &end
    /// =331.15
    /// +331.19
    /// ">Reference</a>
    ThreeStateEnable(Option<EdgeState>),
}

impl ArcCombinational {
    const COMBINATIONAL: &str = "combinational";
    const COMBINATIONAL_RISE: &str = "combinational_rise";
    const COMBINATIONAL_FALL: &str = "combinational_fall";
    const THREE_STATE_DISABLE: &str = "three_state_disable";
    const THREE_STATE_DISABLE_RISE: &str = "three_state_disable_rise";
    const THREE_STATE_DISABLE_FALL: &str = "three_state_disable_fall";
    const THREE_STATE_ENABLE: &str = "three_state_enable";
    const THREE_STATE_ENABLE_RISE: &str = "three_state_enable_rise";
    const THREE_STATE_ENABLE_FALL: &str = "three_state_enable_fall";
}

impl Display for ArcCombinational {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Combinational(edge) => match edge {
                Some(_edge) => match _edge {
                    EdgeState::Fall(_) => write!(f,"{}", Self::COMBINATIONAL_FALL),
                    EdgeState::Rise(_) => write!(f,"{}", Self::COMBINATIONAL_RISE),
                },
                None => write!(f,"{}", Self::COMBINATIONAL),
            },
            Self::ThreeStateDisable(edge) => match edge {
                Some(_edge) => match _edge {
                    EdgeState::Fall(_) => write!(f,"{}", Self::THREE_STATE_DISABLE_FALL),
                    EdgeState::Rise(_) => write!(f,"{}", Self::THREE_STATE_DISABLE_RISE),
                },
                None => write!(f,"{}", Self::THREE_STATE_DISABLE),
            },
            Self::ThreeStateEnable(edge) => match edge {
                Some(_edge) => match _edge {
                    EdgeState::Fall(_) => write!(f,"{}", Self::THREE_STATE_ENABLE_FALL),
                    EdgeState::Rise(_) => write!(f,"{}", Self::THREE_STATE_ENABLE_RISE),
                },
                None => write!(f,"{}", Self::THREE_STATE_ENABLE),
            },
        }
    }
}

impl PartialEq for ArcCombinational {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Combinational(l0), Self::Combinational(r0)) => l0 == r0,
            (Self::ThreeStateDisable(l0), Self::ThreeStateDisable(r0)) => l0 == r0,
            (Self::ThreeStateEnable(l0), Self::ThreeStateEnable(r0)) => l0 == r0,
            _ => false,
        }
    }
}

/// # Sequential Timing Arcs
/// 
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =331.20
/// &end
/// =333.29
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html');
/// </script>
/// 
/// ## Example
/// A sample library with the timing_type attribute and minimum_pulse_width and minimum_period values.
/// ``` liberty
/// library(ASIC) {  
///     ...  
///     delay_model : table_lookup;  
///     ...  
///     lu_table_template(pulse_width_template) {  
///         variable_1 : related_pin_transition;  
///         index_1 ("1.0, 2.0, 3.0");  
///     }  
///     cell(flop) {  
///         ...   
///         pin(CK) {  
///             direction : input;  
///             capacitance : 0.00707171;  
///             timing() {  
///                 timing_type : "min_pulse_width";  
///                 related_pin : "CK";  
///                 ...  
///                 rise_constraint("pulse_width_template") {  
///                     index_1("0.000000, 1.000000, 2.00000");  
///                     values ("6.000000, 6.250000, 7.2500000" );  
///                 }  
///                 fall_constraint("pulse_width_template") {  
///                     index_1("0.000000, 1.000000, 2.00000");  
///                     values ("6.000000, 6.250000, 7.2500000" );  
///                 }  
///             }  
///             timing() {  
///                 timing_type : "minimum_period";  
///                 related_pin : "CK";  
///                 rise_constraint("pulse_width_template") {  
///                     index_1(" .000000, 1.000000, 2.00000");  
///                     values ("6.000000, 6.250000, 7.2500000" );  
///                 }  
///                 fall_constraint("pulse_width_template") {  
///                     index_1("0.000000, 1.000000, 2.00000");  
///                     values ("6.000000, 6.250000, 7.2500000" );  
///                 }  
///             }  
///         }  
///     ...  
///     } /* end cell */ 
/// } /* end library */
/// ```
#[derive(Debug, Clone, Copy)]
pub enum ArcSequential{
    /// `rising_edge`/
    /// `falling_edge`
    /// 
    /// Identifies a timing arc whose output pin is sensitive to a 
    /// `rising`/`falling` signal at the input pin.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =331.21
    /// +331.24
    /// &end
    /// =331.23
    /// +331.26
    /// ">Reference</a>
    Edge(EdgeState),
    /// `preset`
    /// 
    /// Preset arcs affect only the rise arrival time of the arc’s endpoint pin. 
    /// A preset arc implies that you are asserting a logic 1 on the output pin 
    /// when the designated related_pin is asserted.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =331.27
    /// &end
    /// =331.30
    /// ">Reference</a>
    Preset,
    /// `clear`
    /// 
    /// Clear arcs affect only the fall arrival time of the arc’s endpoint pin. 
    /// A clear arc implies that you are asserting a logic 0 on the output pin 
    /// when the designated related_pin is asserted.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =332.2
    /// &end
    /// =332.5
    /// ">Reference</a>
    Clear,
    /// `hold_rising`/
    /// `hold_falling`
    /// 
    /// Designates the `rising`/`falling` edge of the related pin for the hold check.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =332.6
    /// +332.8
    /// &end
    /// =332.7
    /// +332.9
    /// ">Reference</a>
    Hold(EdgeState),
    /// `setup_rising`/
    /// `setup_falling`
    /// 
    /// Designates the `rising`/`falling` edge of the related pin for the setup check 
    /// on clocked elements.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =332.10
    /// +332.13
    /// &end
    /// =332.12
    /// +332.15
    /// ">Reference</a>
    Setup(EdgeState),
    /// `recovery_rising`/
    /// `recovery_falling`
    /// 
    /// Uses the rising edge of the related pin for the recovery time check. 
    /// The clock is `rising-edge-triggered`/`falling-edge-triggered`.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =332.16
    /// +332.19
    /// &end
    /// =332.18
    /// +332.21
    /// ">Reference</a>
    Recovery(EdgeState),
    /// `skew_rising`/
    /// `skew_falling`
    /// 
    /// The timing constraint interval is measured from the rising edge of the 
    /// reference pin (specified in `related_pin`) to a transition edge of the 
    /// parent pin of the timing group. 
    /// 
    /// The `intrinsic_rise`  value is the maximum skew time between the 
    /// reference pin `rising`/`falling` and the parent pin rising. 
    /// 
    /// The `intrinsic_fall` value is the maximum skew time between the 
    /// reference pin `rising`/`falling` and the parent pin falling.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =332.22
    /// +332.28
    /// &end
    /// =332.27
    /// +332.33
    /// ">Reference</a>
    Skew(EdgeState),
    /// `removal_rising`/
    /// `removal_falling`
    /// 
    /// Used when the cell is a `low-enable`/`high-enable` latch or a 
    /// `rising-edge-triggered`/`falling-edge-triggered` flip-flop. 
    /// 
    /// For active-low asynchronous control signals, define the removal 
    /// time with the `intrinsic_rise` attribute. 
    /// 
    /// For active-high asynchronous control signals, define the removal 
    /// time with the `intrinsic_fall` attribute.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =333.2
    /// +333.7
    /// &end
    /// =333.6
    /// +333.11
    /// ">Reference</a>
    Removal(EdgeState),
    /// `min_pulse_width`
    /// 
    /// This value lets you specify the minimum pulse width for a clock pin. 
    /// The timing check is performed on the pin itself, so the `related pin` 
    /// should be the same. You need to specify both rise and fall constraints 
    /// to calculate the high and low pulse widths.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =333.12
    /// &end
    /// =333.16
    /// ">Reference</a>
    MinPulseWidth,
    /// `minimum_period`
    /// 
    /// This value lets you specify the minimum period for a clock pin. 
    /// The timing check is performed on the pin itself, so the `related pin` 
    /// should be the same. You need to specify both rise and fall constraints 
    /// to calculate the minimum clock period. 
    /// 
    /// Rise constraint is characterization data when the clock waveform has a 
    /// rising start edge. Fall constraint is characterization data when the 
    /// start edge of a waveform is falling.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =333.17
    /// &end
    /// =333.23
    /// ">Reference</a>
    MinimumPeriod,
    /// `max_clock_tree_path`/
    /// `min_clock_tree_path`
    /// 
    /// Used in `timing` groups under a clock pin. Defines the `maximum`/`minimum`
    /// clock tree path constraint.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =333.24
    /// +333.27
    /// &end
    /// =333.26
    /// +333.28
    /// ">Reference</a>
    ClockTreePath(MaxMin),
}

impl ArcSequential {
    const RISING_EDGE: &str = "rising_edge";
    const FALLING_EDGE: &str = "falling_edge";
    const PRESET: &str = "preset";
    const CLEAR: &str = "clear";
    const HOLD_RISING: &str = "hold_rising";
    const HOLD_FALLING: &str = "hold_falling";
    const SETUP_RISING: &str = "setup_rising";
    const SETUP_FALLING: &str = "setup_falling";
    const RECOVERY_RISING: &str = "recovery_rising";
    const RECOVERY_FALLING: &str = "recovery_falling";
    const SKEW_RISING: &str = "skew_rising";
    const SKEW_FALLING: &str = "skew_falling";
    const REMOVAL_RISING: &str = "removal_rising";
    const REMOVAL_FALLING: &str = "removal_falling";
    const MIN_PULSE_WIDTH: &str = "min_pulse_width";
    const MINIMUM_PERIOD: &str = "minimum_period";
    const MAX_CLOCK_TREE_PATH: &str = "max_clock_tree_path";
    const MIN_CLOCK_TREE_PATH: &str = "min_clock_tree_path";
}

impl Display for ArcSequential {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Edge(edge) => match edge {
                EdgeState::Fall(_) => write!(f,"{}", Self::FALLING_EDGE),
                EdgeState::Rise(_) => write!(f,"{}", Self::RISING_EDGE),
            },
            Self::Preset => write!(f,"{}", Self::PRESET),
            Self::Clear => write!(f,"{}", Self::CLEAR),
            Self::Hold(edge) => match edge {
                EdgeState::Fall(_) => write!(f,"{}", Self::HOLD_FALLING),
                EdgeState::Rise(_) => write!(f,"{}", Self::HOLD_RISING),
            },
            Self::Setup(edge) => match edge {
                EdgeState::Fall(_) => write!(f,"{}", Self::SETUP_FALLING),
                EdgeState::Rise(_) => write!(f,"{}", Self::SETUP_RISING),
            },
            Self::Recovery(edge) => match edge {
                EdgeState::Fall(_) => write!(f,"{}", Self::RECOVERY_FALLING),
                EdgeState::Rise(_) => write!(f,"{}", Self::RECOVERY_RISING),
            },
            Self::Skew(edge) => match edge {
                EdgeState::Fall(_) => write!(f,"{}", Self::SKEW_FALLING),
                EdgeState::Rise(_) => write!(f,"{}", Self::SKEW_RISING),
            },
            Self::Removal(edge) => match edge {
                EdgeState::Fall(_) => write!(f,"{}", Self::REMOVAL_FALLING),
                EdgeState::Rise(_) => write!(f,"{}", Self::REMOVAL_RISING),
            },
            Self::MinPulseWidth => write!(f,"{}", Self::MIN_PULSE_WIDTH),
            Self::MinimumPeriod => write!(f,"{}", Self::MINIMUM_PERIOD),
            Self::ClockTreePath(max_min) => match max_min {
                MaxMin::Max => write!(f,"{}", Self::MAX_CLOCK_TREE_PATH),
                MaxMin::Min => write!(f,"{}", Self::MIN_CLOCK_TREE_PATH),
            },
        }
    }
}

impl PartialEq for ArcSequential {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Edge(l0), Self::Edge(r0)) => l0 == r0,
            (Self::Preset, Self::Preset) => true,
            (Self::Clear, Self::Clear) => true,
            (Self::Hold(l0), Self::Hold(r0)) => l0 == r0,
            (Self::Setup(l0), Self::Setup(r0)) => l0 == r0,
            (Self::Recovery(l0), Self::Recovery(r0)) => l0 == r0,
            (Self::Skew(l0), Self::Skew(r0)) => l0 == r0,
            (Self::Removal(l0), Self::Removal(r0)) => l0 == r0,
            (Self::MinPulseWidth, Self::MinPulseWidth) => true,
            (Self::MinimumPeriod, Self::MinimumPeriod) => true,
            (Self::ClockTreePath(l0), Self::ClockTreePath(r0)) => l0 == r0,
            _ => false,
        }
    }
}

/// # Nonsequential Timing Arcs
/// 
/// In some nonsequential cells, the setup and hold timing constraints are specified 
/// on the data pin with a nonclock pin as the related pin. It requires the signal of 
/// a pin to be stable for a specified period of time before and after another pin of 
/// the same cell range state so that the cell can function as expected.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =335.29
/// &end
/// =335.37
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone, Copy)]
pub enum ArcNonSequential{
    /// `non_seq_setup_rising`/
    /// `non_seq_setup_falling`
    /// 
    /// Defines (with `non_seq_setup_falling`/`non_seq_setup_rising`) the timing arcs used 
    /// for setup checks between pins with nonsequential behavior. The related pin in 
    /// a timing arc is used for the timing check.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =335.34
    /// +335.38
    /// &end
    /// =335.37
    /// +335.41
    /// ">Reference</a>
    NonSeqSetup(EdgeState),
    /// `non_seq_hold_rising`/
    /// `non_seq_hold_falling`
    /// 
    /// Defines (with `non_seq_hold_falling`/`non_seq_hold_rising`) the timing arcs used 
    /// for hold checks between pins with nonsequential behavior. The related pin in 
    /// a timing arc is used for the timing check.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =335.42
    /// +336.2
    /// &end
    /// =335.45
    /// +336.5
    /// ">Reference</a>
    NonSeqHold(EdgeState),
}

impl ArcNonSequential {
    const NON_SEQ_SETUP_RISING: &str = "non_seq_setup_rising";
    const NON_SEQ_SETUP_FALLING: &str = "non_seq_setup_falling";
    const NON_SEQ_HOLD_RISING: &str = "non_seq_hold_rising";
    const NON_SEQ_HOLD_FALLING: &str = "non_seq_hold_falling";
}

impl Display for ArcNonSequential {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonSeqSetup(edge) => match edge {
                EdgeState::Fall(_) => write!(f,"{}", Self::NON_SEQ_SETUP_FALLING),
                EdgeState::Rise(_) => write!(f,"{}", Self::NON_SEQ_SETUP_RISING),
            },
            Self::NonSeqHold(edge) => match edge {
                EdgeState::Fall(_) => write!(f,"{}", Self::NON_SEQ_HOLD_FALLING),
                EdgeState::Rise(_) => write!(f,"{}", Self::NON_SEQ_HOLD_RISING),
            },
        }
    }
}

impl PartialEq for ArcNonSequential {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NonSeqSetup(l0), Self::NonSeqSetup(r0)) => l0 == r0,
            (Self::NonSeqHold(l0), Self::NonSeqHold(r0)) => l0 == r0,
            _ => false,
        }
    }
}

/// # No-Change Timing Arcs
/// 
/// This feature models the timing requirement of latch devices with latch-enable signals. 
/// The four no-change timing types define the pulse waveforms of both the constrained signal 
/// and the related signal in standard CMOS and nonlinear CMOS delay models. 
/// The information is used in static timing verification during synthesis.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =336.6
/// &end
/// =336.10
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone, Copy)]
pub enum ArcNoChange{
    /// `nochange_high_high`/
    /// `nochange_high_low`/
    /// `nochange_low_high`/
    /// `nochange_low_low`
    /// 
    /// Indicates a `positive`/`positive`/`negative`/`negative` pulse on the constrained pin 
    /// and a `positive`/`negative`/`positive`/`negative` pulse on the related pin.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =336.11
    /// +336.14
    /// +336.17
    /// +336.20
    /// &end
    /// =336.13
    /// +336.16
    /// +336.19
    /// +336.22
    /// ">Reference</a>
    NoChange(StaticState,StaticState),
}

impl ArcNoChange {
    const NOCHANGE_HIGH_HIGH: &str = "nochange_high_high";
    const NOCHANGE_HIGH_LOW: &str = "nochange_high_low";
    const NOCHANGE_LOW_HIGH: &str = "nochange_low_high";
    const NOCHANGE_LOW_LOW: &str = "nochange_low_low";
}

impl Display for ArcNoChange {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoChange(s1, s2) => match (s1,s2) {
                (StaticState::High, StaticState::High) => write!(f,"{}", Self::NOCHANGE_HIGH_HIGH),
                (StaticState::High, StaticState::Low) => write!(f,"{}", Self::NOCHANGE_HIGH_LOW),
                (StaticState::Low, StaticState::High) => write!(f,"{}", Self::NOCHANGE_LOW_HIGH),
                (StaticState::Low, StaticState::Low) => write!(f,"{}", Self::NOCHANGE_LOW_LOW),
            },
        }
    }
}

impl PartialEq for ArcNoChange {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NoChange(l0, l1), Self::NoChange(r0, r1)) => l0 == r0 && l1 == r1,
        }
    }
}

/// The `timing_type` attribute distinguishes between combinational
/// and sequential cells by defining the type of timing arc.
/// If this attribute is not assigned, the cell is considered combinational (Default).
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =330.7
/// &end
/// =330.37
/// ">Reference-Difinition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =320.20
/// &end
/// =320.34
/// ">Reference-Instance</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html');
/// </script>
/// 
/// You must distinguish between combinational and sequential timing types, 
/// because each type serves a different purpose.
/// 
/// The Design Compiler tool uses the combinational timing arcs information to calculate 
/// the physical delays in timing propagation and to trace paths. The timing analyzer uses 
/// path-tracing arcs for circuit timing analysis. 
/// 
/// The Design Compiler tool uses the sequential timing arcs information to determine 
/// rule-based design optimization constraints. More information on optimization 
/// constraints is available in the Design Compiler documentation. 
/// 
/// The following sections show the `timing_type` attribute values for the following 
/// timing arcs. For information about when to use the different types, see the 
/// *Synopsys Liberty User Guide*.
/// 
/// + [Combinational](crate::timing::ArcCombinational)
/// + [Sequential](crate::timing::ArcSequential)
/// + [NonSequential](crate::timing::ArcNonSequential)
/// + [NoChange](crate::timing::ArcNoChange)
/// 
/// #### Syntax
/// `timing_type : combinational | combinational_rise | combinational_fall | three_state_disable | 
/// three_state_disable_rise | three_state_disable_fall | three_state_enable | three_state_enable_rise | 
/// three_state_enable_fall | rising_edge | falling_edge | preset | clear | hold_rising | hold_falling | 
/// setup_rising | setup_falling | recovery_rising | recovery_falling | skew_rising | skew_falling | 
/// removal_rising | removal_falling | min_pulse_width | minimum_period | max_clock_tree_path |
/// min_clock_tree_path |non_seq_setup_rising | non_seq_setup_falling | non_seq_hold_rising | 
/// non_seq_hold_falling | nochange_high_high | nochange_high_low | nochange_low_high | nochange_low_low ;`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimingType {
    /// [Combinational](crate::timing::ArcCombinational)
    Combinational(ArcCombinational),
    /// [Sequential](crate::timing::ArcSequential)
    Sequential(ArcSequential),
    /// [NonSequential](crate::timing::ArcNonSequential)
    NonSequential(ArcNonSequential),
    /// [NoChange](crate::timing::ArcNoChange)
    NoChange(ArcNoChange),
}

impl Default for TimingType {
    /// combinational
    #[inline]
    fn default() -> Self {
        Self::COMBINATIONAL
    }
}
impl Display for TimingType {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimingType::Combinational(t) => t.fmt(f),
            TimingType::Sequential(t) => t.fmt(f),
            TimingType::NonSequential(t) => t.fmt(f),
            TimingType::NoChange(t) => t.fmt(f),
        }
    }
}

impl TimingType {
    /// COMBINATIONAL
    pub const COMBINATIONAL: Self = Self::Combinational(ArcCombinational::Combinational(None));
    /// COMBINATIONAL_RISE
    pub const COMBINATIONAL_RISE: Self = Self::Combinational(ArcCombinational::Combinational(Some(EdgeState::Rise(None))));
    /// COMBINATIONAL_FALL
    pub const COMBINATIONAL_FALL: Self = Self::Combinational(ArcCombinational::Combinational(Some(EdgeState::Fall(None))));
    /// THREE_STATE_DISABLE
    pub const THREE_STATE_DISABLE: Self = Self::Combinational(ArcCombinational::ThreeStateDisable(None));
    /// THREE_STATE_DISABLE_RISE
    pub const THREE_STATE_DISABLE_RISE: Self = Self::Combinational(ArcCombinational::ThreeStateDisable(Some(EdgeState::Rise(None))));
    /// THREE_STATE_DISABLE_FALL
    pub const THREE_STATE_DISABLE_FALL: Self = Self::Combinational(ArcCombinational::ThreeStateDisable(Some(EdgeState::Fall(None))));
    /// THREE_STATE_ENABLE
    pub const THREE_STATE_ENABLE: Self = Self::Combinational(ArcCombinational::ThreeStateEnable(None));
    /// THREE_STATE_ENABLE_RISE
    pub const THREE_STATE_ENABLE_RISE: Self = Self::Combinational(ArcCombinational::ThreeStateEnable(Some(EdgeState::Rise(None))));
    /// THREE_STATE_ENABLE_FALL
    pub const THREE_STATE_ENABLE_FALL: Self = Self::Combinational(ArcCombinational::ThreeStateEnable(Some(EdgeState::Fall(None))));
    /// RISING_EDGE
    pub const RISING_EDGE: Self = Self::Sequential(ArcSequential::Edge(EdgeState::Rise(None)));
    /// FALLING_EDGE
    pub const FALLING_EDGE: Self = Self::Sequential(ArcSequential::Edge(EdgeState::Fall(None)));
    /// PRESET
    pub const PRESET: Self = Self::Sequential(ArcSequential::Preset);
    /// CLEAR
    pub const CLEAR: Self = Self::Sequential(ArcSequential::Clear);
    /// HOLD_RISING
    pub const HOLD_RISING: Self = Self::Sequential(ArcSequential::Hold(EdgeState::Rise(None)));
    /// HOLD_FALLING
    pub const HOLD_FALLING: Self = Self::Sequential(ArcSequential::Hold(EdgeState::Fall(None)));
    /// SETUP_RISING
    pub const SETUP_RISING: Self = Self::Sequential(ArcSequential::Setup(EdgeState::Rise(None)));
    /// SETUP_FALLING
    pub const SETUP_FALLING: Self = Self::Sequential(ArcSequential::Setup(EdgeState::Fall(None)));
    /// RECOVERY_RISING
    pub const RECOVERY_RISING: Self = Self::Sequential(ArcSequential::Recovery(EdgeState::Rise(None)));
    /// RECOVERY_FALLING
    pub const RECOVERY_FALLING: Self = Self::Sequential(ArcSequential::Recovery(EdgeState::Fall(None)));
    /// SKEW_RISING
    pub const SKEW_RISING: Self = Self::Sequential(ArcSequential::Skew(EdgeState::Rise(None)));
    /// SKEW_FALLING
    pub const SKEW_FALLING: Self = Self::Sequential(ArcSequential::Skew(EdgeState::Fall(None)));
    /// REMOVAL_RISING
    pub const REMOVAL_RISING: Self = Self::Sequential(ArcSequential::Removal(EdgeState::Rise(None)));
    /// REMOVAL_FALLING
    pub const REMOVAL_FALLING: Self = Self::Sequential(ArcSequential::Removal(EdgeState::Fall(None)));
    /// MIN_PULSE_WIDTH
    pub const MIN_PULSE_WIDTH: Self = Self::Sequential(ArcSequential::MinPulseWidth);
    /// MINIMUM_PERIOD
    pub const MINIMUM_PERIOD: Self = Self::Sequential(ArcSequential::MinimumPeriod);
    /// MAX_CLOCK_TREE_PATH
    pub const MAX_CLOCK_TREE_PATH: Self = Self::Sequential(ArcSequential::ClockTreePath(MaxMin::Max));
    /// MIN_CLOCK_TREE_PATH
    pub const MIN_CLOCK_TREE_PATH: Self = Self::Sequential(ArcSequential::ClockTreePath(MaxMin::Min));
    /// NON_SEQ_SETUP_RISING
    pub const NON_SEQ_SETUP_RISING: Self = Self::NonSequential(ArcNonSequential::NonSeqSetup(EdgeState::Rise(None)));
    /// NON_SEQ_SETUP_FALLING
    pub const NON_SEQ_SETUP_FALLING: Self = Self::NonSequential(ArcNonSequential::NonSeqSetup(EdgeState::Fall(None)));
    /// NON_SEQ_HOLD_RISING
    pub const NON_SEQ_HOLD_RISING: Self = Self::NonSequential(ArcNonSequential::NonSeqHold(EdgeState::Rise(None)));
    /// NON_SEQ_HOLD_FALLING
    pub const NON_SEQ_HOLD_FALLING: Self = Self::NonSequential(ArcNonSequential::NonSeqHold(EdgeState::Fall(None)));
    /// NOCHANGE_HIGH_HIGH
    pub const NOCHANGE_HIGH_HIGH: Self = Self::NoChange(ArcNoChange::NoChange(StaticState::High, StaticState::High));
    /// NOCHANGE_HIGH_LOW
    pub const NOCHANGE_HIGH_LOW: Self = Self::NoChange(ArcNoChange::NoChange(StaticState::High, StaticState::Low));
    /// NOCHANGE_LOW_HIGH
    pub const NOCHANGE_LOW_HIGH: Self = Self::NoChange(ArcNoChange::NoChange(StaticState::Low, StaticState::High));
    /// NOCHANGE_LOW_LOW
    pub const NOCHANGE_LOW_LOW: Self = Self::NoChange(ArcNoChange::NoChange(StaticState::Low, StaticState::Low));
}

impl TimingType {
    const LIST: [Self;35] = [
        Self::COMBINATIONAL,
        Self::COMBINATIONAL_RISE,
        Self::COMBINATIONAL_FALL,
        Self::THREE_STATE_DISABLE,
        Self::THREE_STATE_DISABLE_RISE,
        Self::THREE_STATE_DISABLE_FALL,
        Self::THREE_STATE_ENABLE,
        Self::THREE_STATE_ENABLE_RISE,
        Self::THREE_STATE_ENABLE_FALL,
        Self::RISING_EDGE,
        Self::FALLING_EDGE,
        Self::PRESET,
        Self::CLEAR,
        Self::HOLD_RISING,
        Self::HOLD_FALLING,
        Self::SETUP_RISING,
        Self::SETUP_FALLING,
        Self::RECOVERY_RISING,
        Self::RECOVERY_FALLING,
        Self::SKEW_RISING,
        Self::SKEW_FALLING,
        Self::REMOVAL_RISING,
        Self::REMOVAL_FALLING,
        Self::MIN_PULSE_WIDTH,
        Self::MINIMUM_PERIOD,
        Self::MAX_CLOCK_TREE_PATH,
        Self::MIN_CLOCK_TREE_PATH,
        Self::NON_SEQ_SETUP_RISING,
        Self::NON_SEQ_SETUP_FALLING,
        Self::NON_SEQ_HOLD_RISING,
        Self::NON_SEQ_HOLD_FALLING,
        Self::NOCHANGE_HIGH_HIGH,
        Self::NOCHANGE_HIGH_LOW,
        Self::NOCHANGE_LOW_HIGH,
        Self::NOCHANGE_LOW_LOW,
    ];
    /// iter
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::LIST.iter().copied()
    }
}

impl FromStr for TimingType {
    type Err=std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ArcCombinational::COMBINATIONAL => Ok(Self::COMBINATIONAL),
            ArcCombinational::COMBINATIONAL_RISE => Ok(Self::COMBINATIONAL_RISE),
            ArcCombinational::COMBINATIONAL_FALL => Ok(Self::COMBINATIONAL_FALL),
            ArcCombinational::THREE_STATE_DISABLE => Ok(Self::THREE_STATE_DISABLE),
            ArcCombinational::THREE_STATE_DISABLE_RISE => Ok(Self::THREE_STATE_DISABLE_RISE),
            ArcCombinational::THREE_STATE_DISABLE_FALL => Ok(Self::THREE_STATE_DISABLE_FALL),
            ArcCombinational::THREE_STATE_ENABLE => Ok(Self::THREE_STATE_ENABLE),
            ArcCombinational::THREE_STATE_ENABLE_RISE => Ok(Self::THREE_STATE_ENABLE_RISE),
            ArcCombinational::THREE_STATE_ENABLE_FALL => Ok(Self::THREE_STATE_ENABLE_FALL),
            ArcSequential::RISING_EDGE => Ok(Self::RISING_EDGE),
            ArcSequential::FALLING_EDGE => Ok(Self::FALLING_EDGE),
            ArcSequential::PRESET => Ok(Self::PRESET),
            ArcSequential::CLEAR => Ok(Self::CLEAR),
            ArcSequential::HOLD_RISING => Ok(Self::HOLD_RISING),
            ArcSequential::HOLD_FALLING => Ok(Self::HOLD_FALLING),
            ArcSequential::SETUP_RISING => Ok(Self::SETUP_RISING),
            ArcSequential::SETUP_FALLING => Ok(Self::SETUP_FALLING),
            ArcSequential::RECOVERY_RISING => Ok(Self::RECOVERY_RISING),
            ArcSequential::RECOVERY_FALLING => Ok(Self::RECOVERY_FALLING),
            ArcSequential::SKEW_RISING => Ok(Self::SKEW_RISING),
            ArcSequential::SKEW_FALLING => Ok(Self::SKEW_FALLING),
            ArcSequential::REMOVAL_RISING => Ok(Self::REMOVAL_RISING),
            ArcSequential::REMOVAL_FALLING => Ok(Self::REMOVAL_FALLING),
            ArcSequential::MIN_PULSE_WIDTH => Ok(Self::MIN_PULSE_WIDTH),
            ArcSequential::MINIMUM_PERIOD => Ok(Self::MINIMUM_PERIOD),
            ArcSequential::MAX_CLOCK_TREE_PATH => Ok(Self::MAX_CLOCK_TREE_PATH),
            ArcSequential::MIN_CLOCK_TREE_PATH => Ok(Self::MIN_CLOCK_TREE_PATH),
            ArcNonSequential::NON_SEQ_SETUP_RISING => Ok(Self::NON_SEQ_SETUP_RISING),
            ArcNonSequential::NON_SEQ_SETUP_FALLING => Ok(Self::NON_SEQ_SETUP_FALLING),
            ArcNonSequential::NON_SEQ_HOLD_RISING => Ok(Self::NON_SEQ_HOLD_RISING),
            ArcNonSequential::NON_SEQ_HOLD_FALLING => Ok(Self::NON_SEQ_HOLD_FALLING),
            ArcNoChange::NOCHANGE_HIGH_HIGH => Ok(Self::NOCHANGE_HIGH_HIGH),
            ArcNoChange::NOCHANGE_HIGH_LOW => Ok(Self::NOCHANGE_HIGH_LOW),
            ArcNoChange::NOCHANGE_LOW_HIGH => Ok(Self::NOCHANGE_LOW_HIGH),
            ArcNoChange::NOCHANGE_LOW_LOW => Ok(Self::NOCHANGE_LOW_LOW),
            _ => Err(std::fmt::Error)
        }
    }
}

#[cfg(test)]
mod test{
    use std::str::FromStr;

    use super::TimingType;

    #[test]
    fn iter() {
        for t in TimingType::iter(){
            println!("{t}\t{t:?}");
            assert_eq!(Ok(t),TimingType::from_str(&format!("{t}")));
        }
    }
}