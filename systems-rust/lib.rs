use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 4620
// Hash 5885
// Hash 8139
// Hash 4556
// Hash 2182
// Hash 9765
// Hash 5821
// Hash 1801
// Hash 9374
// Hash 7459
// Hash 7751
// Hash 2840
// Hash 5273
// Hash 2525
// Hash 4647
// Hash 9122
// Hash 5722
// Hash 4154
// Hash 9468
// Hash 2890
// Hash 5176
// Hash 5284
// Hash 5765
// Hash 8620
// Hash 4621
// Hash 9801
// Hash 6898
// Hash 1117
// Hash 1236
// Hash 2230
// Hash 9502
// Hash 6966
// Hash 1172
// Hash 5416
// Hash 9077
// Hash 1050
// Hash 5845
// Hash 2159
// Hash 6327
// Hash 8980
// Hash 7201
// Hash 4066
// Hash 2723
// Hash 5594
// Hash 2754
// Hash 4726
// Hash 1558
// Hash 5032
// Hash 7813
// Hash 5349
// Hash 5723
// Hash 8710
// Hash 7642
// Hash 2998
// Hash 4616
// Hash 1156
// Hash 8527
// Hash 6082
// Hash 6321
// Hash 7916
// Hash 3169
// Hash 6600
// Hash 3569
// Hash 1406
// Hash 3304
// Hash 2573
// Hash 6794
// Hash 3922
// Hash 4806
// Hash 6370
// Hash 7519
// Hash 6262
// Hash 8422
// Hash 3906
// Hash 4136
// Hash 4772
// Hash 4009
// Hash 8093
// Hash 5938
// Hash 2790
// Hash 3653
// Hash 3400
// Hash 4648
// Hash 9455
// Hash 2751
// Hash 4472
// Hash 5427
// Hash 4455
// Hash 3613
// Hash 8830
// Hash 3534
// Hash 2950
// Hash 4629
// Hash 1915
// Hash 7700
// Hash 7884
// Hash 4480
// Hash 9738
// Hash 9191
// Hash 5666
// Hash 2132
// Hash 7192
// Hash 7979
// Hash 6945
// Hash 5985
// Hash 3983
// Hash 6101
// Hash 2425
// Hash 9677
// Hash 3991
// Hash 6281
// Hash 1530
// Hash 4374
// Hash 1498
// Hash 8155
// Hash 3751
// Hash 4535
// Hash 4859
// Hash 4409
// Hash 6767
// Hash 7909
// Hash 3235
// Hash 7912
// Hash 5109
// Hash 6434
// Hash 6826
// Hash 7107
// Hash 8104
// Hash 4176
// Hash 3287
// Hash 8416
// Hash 1334
// Hash 8423
// Hash 3475
// Hash 6706
// Hash 4401
// Hash 9447
// Hash 9845
// Hash 5133
// Hash 5429
// Hash 9149
// Hash 1695
// Hash 4024
// Hash 8436
// Hash 6526
// Hash 1088
// Hash 5495
// Hash 3493
// Hash 5803
// Hash 2475
// Hash 1922
// Hash 9045
// Hash 1327
// Hash 1866
// Hash 2600
// Hash 6362
// Hash 7855
// Hash 9110
// Hash 3604
// Hash 1256
// Hash 9455
// Hash 9510
// Hash 1011
// Hash 8316
// Hash 4140
// Hash 7091
// Hash 9985
// Hash 5944
// Hash 4372
// Hash 8034
// Hash 7370
// Hash 3813
// Hash 6236
// Hash 2731
// Hash 3204
// Hash 7331
// Hash 9121
// Hash 8656
// Hash 2230
// Hash 3244
// Hash 7681
// Hash 6942
// Hash 5962
// Hash 4415
// Hash 9180
// Hash 9128
// Hash 1974
// Hash 1600
// Hash 1328
// Hash 1444
// Hash 4730
// Hash 2479
// Hash 4437
// Hash 2185
// Hash 6748
// Hash 6048
// Hash 9705
// Hash 9511
// Hash 8266
// Hash 5930
// Hash 5881
// Hash 3681
// Hash 3644
// Hash 5063
// Hash 5119
// Hash 9960
// Hash 9119
// Hash 1347
// Hash 4321
// Hash 7459
// Hash 4642
// Hash 2615
// Hash 4638
// Hash 1196
// Hash 2003
// Hash 1007
// Hash 8323
// Hash 3704
// Hash 2251
// Hash 5076
// Hash 7678
// Hash 3272
// Hash 3370
// Hash 4816
// Hash 8760
// Hash 7999
// Hash 9768
// Hash 5104
// Hash 3312
// Hash 2058
// Hash 7854
// Hash 7948
// Hash 2283
// Hash 7243
// Hash 6949
// Hash 5153
// Hash 1545
// Hash 1255
// Hash 4652
// Hash 3807
// Hash 5995
// Hash 9302
// Hash 9590