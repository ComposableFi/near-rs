use std::io;

use near_primitives::views::LightClientBlockView as NearLightClientBlockView;
use serde::Deserialize;

use crate::client_proof::LightClientBlockView;

#[derive(Debug, Deserialize)]
struct ResultFromRpc {
    pub result: NearLightClientBlockView,
}

pub fn get_client_block_view(client_block_response: &str) -> io::Result<LightClientBlockView> {
    Ok(
        serde_json::from_str::<ResultFromRpc>(client_block_response)?
            .result
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{json_types::Base58CryptoHash, CryptoHash as JSONCryptoHash};

    const CLIENT_BLOCK_RESPONSE: &'static str = r#"
    {
        "jsonrpc": "2.0",
        "result": {
            "approvals_after_next": [
                "ed25519:2t3LpSERvt1BACt2KTPq6kXiqqEq2bwwcWm93zqmkdTU84d9R7hCgVtntHm72rPHGpDzKrqJZbYrSHiXsmoHBpAH",
                "ed25519:2t3LpSERvt1BACt2KTPq6kXiqqEq2bwwcWm93zqmkdTU84d9R7hCgVtntHm72rPHGpDzKrqJZbYrSHiXsmoHBpAH",
                "ed25519:2t3LpSERvt1BACt2KTPq6kXiqqEq2bwwcWm93zqmkdTU84d9R7hCgVtntHm72rPHGpDzKrqJZbYrSHiXsmoHBpAH",
                "ed25519:2t3LpSERvt1BACt2KTPq6kXiqqEq2bwwcWm93zqmkdTU84d9R7hCgVtntHm72rPHGpDzKrqJZbYrSHiXsmoHBpAH",
                "ed25519:RoVqn1cacyvGLSWeybCrg3v6PebsUgXoAxnbvkuhMoBabz6Tfc1BHqREfThv65Ti9vVxQTWeWdYJrqZ5v8SZLmx",
                "ed25519:438Ym82GkS4jBvLwHoHnT6gQRwh2GHUQgsK7mq2UaToXAJs72jCkqTTvYyetF7HLCjVAHoAnvv3VLfN1N2F4uiVU",
                "ed25519:2naBonh53bm3V9SUf9vMuRho1HkzFy4qEgjauVKmgiSUFGEpAbkZzuybMsHta65GHsZA4fhQ8UaBz6wfAkuMDxws",
                "ed25519:9Y54E9LBXBp8QfE5gTNBtZYtwD5cbwf3zhQ57BwkeUg2Hi1yW4zJ7iBjf4gSNXcAcEfeqvGAkRkBaXDDvuykMUT",
                "ed25519:4nyMP7Y5HXmYRCGd1qo24EMw2fwwmYi3Y3RYVhqV75uKHjxcwqHGiiaJC797tJs2DYH3WxuHZDEKnpYDvLJ3Tvsw",
                "ed25519:4yZHDkwf9t8d1g1MSnLC663pQ49ekKAjYKtng8sSmeL9hsoGTiEZBV3Qn83Y4BfjNixBy324CVGZJ9kD1ZsqbYr9",
                "ed25519:izQ6NgxwsNBEu8ixi4Aor3REjVZgVSqpVUQLst8gSpswfadJCPsvCC9qZufcWXQmD8Jfq5TtERJjSsVQLd6wSXs",
                "ed25519:259bLojqsGVzwEayrt22soLrW8o7FhkRsmSrhARJUpRFJLdeKdT13UintSvQESG3mg4JLLX3i2Vf9w7Hq98sJpnN",
                null,
                "ed25519:grJ7HJWpcY5cFAquqe3nTjHxBj27NqRycefovMTbtKTDnzfZMeWbA8pG9o6AuAstCgzeac2BuHBwEb15PYHw3o8",
                "ed25519:54AjvF9kJLpGSWqk3j5DUWyzdmJ58eCSsKZL2CAgybehHAymRyAvm4boiLR6knTxomngJiG6RXHGXbdRJgRsmENp",
                "ed25519:4Uv3W6MBej3oquYK3McJSW4og7NCGdELgYZuU5Zn1cJ5LStuPTLkXBBc4zR9c16aW95RwGtdmYdwvRfyRQuBkMGu",
                "ed25519:35pGjMHcUyP6qJZaBokaMsmMFs7w5T38dJPX7XHkdFeHhWsTmQP9nL1fodZsWYgpujksbSxjyR68tY63F6ii5f88",
                "ed25519:2ErpQhE1oHndU7pHHd8QqwKWzGXCDHfxt8CZBwfvgcUkZYXFwTAZdt8PVoex61o6LNA2YXHSEVhHUYL6jz94gg9u",
                "ed25519:29qaqxXXpiJDc1EEfu7MPpY59tcmDkiHG19WLVi63AjRWJCWvsETCQwapkaYi6saBssYMUD7RS3EYWW7Ag51CDq3",
                "ed25519:5QATkwJao7XiKLZzK74nYbCviq5nZ6nuykLErYcQtYWbfoH8MnXJnXR9Rb9b47iDaaDEfsGtn1S2HZuvCC38UZQo",
                "ed25519:5rRfQk3DBwp2tvCwJtvjPt7xttvsBHSsxNDu1smx3ium7xuDHra1xcFCffpm7ccFxuKfqeSQkUNU6qt7fBJR3Eda",
                null,
                null,
                "ed25519:42PiQ44gz1rr3UcAbjyfA3kePgsptBYznoaWpD9Ao8eBVEeu7sRABFdcxsC2adWYpB2XuVsAXH8wapYur2micKUW",
                "ed25519:4Ho3SgyqwJq8vqSgdyJS83zdTE7eKA9HFrHyh9s3mCmsVeFCXwnQfJ4BuTMBdm1LJa7arZSesfTAwxfmXWryk89e",
                "ed25519:WmTmDVoWCTNKEaBkYCoNXWFkToFhGCsUduLke6icPSdzPpupK9CF5c63adS7sQ9vDUtrgXEo1vpeM7THi3FQRCp",
                "ed25519:bTbMzzmc3sUbjg8aPHrR22E4HTJURV458vhoZQJYfgbhRphY5vjK79NhAvz4xyCzxkyxASpidPm8XsKaP46mqQ9",
                "ed25519:3WWN46nYYyUyPVEGoLjSZPL4WQWzrd6CFtvBDGrwzpFcTBS7UhyiB3HFxiYcdirKvvALPwoPzdDRGjmHkEuzESWY",
                "ed25519:5aSiBdTLRxb27bSPMjPtBZaGDcf3wbyq725QT5EDvSTFvweb7E8taVnpRhEnytKhtes5ygbrBaAHWmKei7xdNR8o",
                "ed25519:2brq7tnzcf4qfx7po1uR96Mmiic2Cc9TEfM78fVGta1eevD78rQ8qTcpiz88CnePLkFHPrEc8t2px4iiU7cEmoqU",
                "ed25519:3FU7iKQWcCFzETRhDyp6zedVe5Ugzq97QMcypE5WE3ZxATuKuGqerm828xAErx4AoADmW2E3b8yWdmQMX8C2LTS8",
                "ed25519:4ax2bYNWcnjPh33Usjcj1HLfY3o134HCk9MSWss276wggksTL4Jq59YPSH5KsETszsTKWnMAT2b92hPtSUZqLTEp",
                null,
                "ed25519:2DdKg9LvdXajeJfQuZuCE2gJGQE7EHa1uazPEjtQTo6QSp29JKNCrmMN7NdoXBZc7RmBANH441VeKKQ7zJZhx2ER",
                "ed25519:67UNzC6rCtn4jQ4RhvURMA4MTpAbj5tihRsSrHKAkQ6kSxkTfARJaq66Z5DQ7BRp5priB9vHSTxurBKmNC1sXMPi",
                "ed25519:upMr4PvvnKun2e8zcjvESZY3hcRf5RdHnP8SYHuMsXugtT8cnnzFD694262oSuLAAAUof5Cyp7cmtVz2cKLyh9i",
                "ed25519:KSmBW8rkMVXrzwvHj3vk3DqurEcodha35auHW6z83i7RPSRM3LBqhyzQAKtmtFrLWhKUweaPhJwfykD2m8uCs1g",
                null,
                "ed25519:4dnQ17MjKXHmV4EzvH3x67Cb8UusCezgpdHcoUuu7M6jayJq4AbuzEdxKJS2RNpJM55thZRZkcAgkjb34zNYoi6d",
                "ed25519:2evZLyx1HQHy8QuJ5AjZ4LV5ixgQF4RoXjjTQ58ekuQ4NqjrYiY89UXBH9nR4oQfgSzm3beUQiLfjrDzQG5dBdVQ",
                "ed25519:29a84cBDPNrPevRW3QGmfJceypqv5yipADskeAPU7i2dPenLeFhNEGAz4i3pk1Vsuww23kKS43jNey1e3QRZy2iL",
                "ed25519:3jByH4Rg4kvpkuP6nchhpXsEwcgaW9RgWRwW4w4gGnj5xF1pQeohfxjaydBgktMsqDyfHUn5tNP4vfpfqyG6uZb3",
                "ed25519:34yZaVSXLbzY5S1ZjhdZGSUsCRaDtHbCBjVWKS1Fq5u2s71YEjmmJZrLzvG6fmpHYGxZHheW3GNmSGwxW9jw29tP",
                "ed25519:5mg8DioY2H9WYFSXxaHfo4GK6NxxKNCJmMz1uLmyXhncvXYNUJWJ3TtfMaRob1WKgmyFbTAzjrAhoJWfEqzdgK9w",
                "ed25519:2omhuygDKZeb2cAFvzt417B27MhtP874tc23USZfNtDumJmkQgam9LoRjgAH53EutuMhyWa5eg7mpXSCX4PqFxxi",
                "ed25519:aVHieDkvprFCj1UW6vT5SrtHR5K3tjTqCcZRdq1yoJStqVUUEST6MiiHVf3fX367MLk238M5eAfVczC8LKpuR5o",
                null,
                null,
                null,
                "ed25519:AodYtt8tzEai4osiTyQwzrDPXuXvVry4bGXsnDy7bCpRrQjBL6SMtsvrBqA9PGXRtHKQLNQqVAB4VVfZRw1mehg",
                null,
                null,
                null,
                "ed25519:2NQVifuczKzmzYvQGHrGFnzRJkDpcHJKNswtKFT8MZA4GPkDVdUAGAM9EudmVHh8a9JusdBYGxJRdHLu1xRvYbBF",
                "ed25519:3iBn3QBv7PQ7mTTpNhas26oEyMmCdWkFm4k9a5wLiVMCqADT9Sou2sMAnqPPfYqgjM18EKa1tEu5cg9nVWKJYXVJ",
                "ed25519:4ifwqWg4GPqNMLoMMK4AbJtEP4mxs6T7VGPZyPjuMcYVXp9f4CvPDv8cUwEVEGE6QFGxASwm3i2jmDSLedikPb5W",
                "ed25519:4mRccFQXNWTpd9Ho47TW822KeJ49F3JWy83LxUxpnX3csDMePb71u5uiZxqVKcCq6hTDdobidbzxfB6gWAqtF3tP",
                "ed25519:34eZWUnffMERsbCqyXmdtByGXeDc9GkGqTUVHgsmBprbSn7sJ8BM4Sv4efpo4xxVYnnbjnuCiGVF7B3pWV5jzMS3",
                null,
                "ed25519:4U88kcf8WePh6xXgYukvuJ8euqyRixE9ETeNQTpsAdYCpSbBbGgZAFa528eXutZJCcEmmEzszUtCdnZP86X97Dwr",
                null,
                "ed25519:42GDzQAgvxTxgWbX6ziS392GiCzkHMcRW5bho85pLrdfA6R9hLyQV9riFPsy4h7kDCxpkgcCicN8CvejCajyU6jN",
                "ed25519:63xXTd87MnGH1TBFBALNKUe9XkaH3HfyyXqs7E6CWRphc7KxsbCBP1ACgSrK6gW7pLn2euzQEXQpsEr5uLRJek2o",
                null,
                "ed25519:5w2xEcvcJq3sXAFXNd2PWDvhP7zZ41gUhs7DAzuRxZWUDhhUpHTJB9Jjtjnh3HecfYb4KSZMaMBWis5typjFaeuC"
            ],
            "inner_lite": {
                "block_merkle_root": "97q8YFmBHhzd4wJ1C9dC75aDmGt1wCEZahU6qPtsguMi",
                "epoch_id": "9TBdDXkBRVXao8kwt3SdA8ArEjDb3i5hKgmaPCidPheR",
                "height": 86380519,
                "next_bp_hash": "i8qwFBBJDts2puLQHVy8GE6mnuCbPd2GnnTw5Cu1MEw",
                "next_epoch_id": "5iyA2nJxvV4CVJwmfkK72X2f8s57shB2E174vWAPwHB7",
                "outcome_root": "BkvkfvQziYPUfaLYHEAyfnqehqZJiPybE9CzwXTQug2e",
                "prev_state_root": "2CymQ1Mdkmrodd3XXhYhYtbUHQJHfuFZu3PbM1SQr9su",
                "timestamp": 1648731805871375246,
                "timestamp_nanosec": "1648731805871375246"
            },
            "inner_rest_hash": "6TEBR8AQ7BnJEWjRmHLDEujqEYwqW91xMfwidwsiARWc",
            "next_block_inner_hash": "B6eEz4Uw82qDY7tkeWSm3eJYCUggn8VpdD62TLv5nVYD",
            "next_bps": [
                {
                    "account_id": "node1",
                    "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                    "stake": "22896126325387195980109277045981",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "node0",
                    "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                    "stake": "16905641685786935339009965772241",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "node2",
                    "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                    "stake": "16855079063935511642921953635551",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "node3",
                    "public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
                    "stake": "8557952899258202284428022875823",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "legends.pool.f863973.m0",
                    "public_key": "ed25519:AhQ6sUifJYgjqarXSAzdDZU9ZixpUesP9JEH1Vr7NbaF",
                    "stake": "5779886769756389765443485143816",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "staked.pool.f863973.m0",
                    "public_key": "ed25519:D2afKYVaKQ1LGiWbMAZRfkKLgqimTR74wvtESvjx5Ft2",
                    "stake": "4549687592623105156004771422926",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "masternode24.pool.f863973.m0",
                    "public_key": "ed25519:9E3JvrQN6VGDGg1WJ3TjBsNyfmrU6kncBcDvvJLj6qHr",
                    "stake": "3408454314030529535871839346421",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "01node.pool.f863973.m0",
                    "public_key": "ed25519:3iNqnvBgxJPXCxu6hNdvJso1PEAc1miAD35KQMBCA3aL",
                    "stake": "3054180112709587862994444937717",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "p2p.pool.f863973.m0",
                    "public_key": "ed25519:4ie5979JdSR4f7MRAG58eghRxndVoKnAYAKa1PLoMYSS",
                    "stake": "2951564608404724101805020396835",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "nodeasy.pool.f863973.m0",
                    "public_key": "ed25519:25Dhg8NBvQhsVTuugav3t1To1X1zKiomDmnh8yN9hHMb",
                    "stake": "1571514758154822682528033715037",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "tribe-pool.pool.f863973.m0",
                    "public_key": "ed25519:CRS4HTSAeiP8FKD3c3ZrCL5pC92Mu1LQaWj22keThwFY",
                    "stake": "1425886033800536187630765974327",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "chorusone.pool.f863973.m0",
                    "public_key": "ed25519:3TkUuDpzrq75KtJhkuLfNNJBPHR5QEWpDxrter3znwto",
                    "stake": "1275860595840145052147470169613",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "hotones.pool.f863973.m0",
                    "public_key": "ed25519:2fc5xtbafKiLtxHskoPL2x7BpijxSZcwcAjzXceaxxWt",
                    "stake": "1270522721570778105970483709507",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "foundryusa.pool.f863973.m0",
                    "public_key": "ed25519:ABGnMW8c87ZKWxvZLLWgvrNe72HN7UoSf4cTBxCHbEE5",
                    "stake": "1253030070453697820587175622599",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lunanova2.pool.f863973.m0",
                    "public_key": "ed25519:9Jv6e9Kye4wM9EL1XJvXY8CYsLi1HLdRKnTzXBQY44w9",
                    "stake": "1244539690165514924847443674519",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "chorus-one.pool.f863973.m0",
                    "public_key": "ed25519:6LFwyEEsqhuDxorWfsKcPPs324zLWTaoqk4o6RDXN7Qc",
                    "stake": "1107854853164230422557488743440",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "ni.pool.f863973.m0",
                    "public_key": "ed25519:GfCfFkLk2twbAWdsS3tr7C2eaiHN3znSfbshS5e8NqBS",
                    "stake": "1074406780988563035968401979532",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "cryptogarik.pool.f863973.m0",
                    "public_key": "ed25519:FyFYc2MVwgitVf4NDLawxVoiwUZ1gYsxGesGPvaZcv6j",
                    "stake": "838704166965912204921121240626",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "pathrocknetwork.pool.f863973.m0",
                    "public_key": "ed25519:CGzLGZEMb84nRSRZ7Au1ETAoQyN7SQXQi55fYafXq736",
                    "stake": "747792475305692775734809426333",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "stakely_v2.pool.f863973.m0",
                    "public_key": "ed25519:7BanKZKGvFjK5Yy83gfJ71vPhqRwsDDyVHrV2FMJCUWr",
                    "stake": "733076097292901021692922082563",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "solidstate.pool.f863973.m0",
                    "public_key": "ed25519:DTDhqoMXDWhKedWpH7DPvR6dPDcXrk5pTHJw2bkFFvQy",
                    "stake": "713547661542757440290927919080",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "gdtesting.pool.f863973.m0",
                    "public_key": "ed25519:7jDiTygfQEUMJsddzmY6CDt44ns4MbtqvimPCp4CT2Ec",
                    "stake": "709868346441730435916131564174",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "aurora.pool.f863973.m0",
                    "public_key": "ed25519:9c7mczZpNzJz98V1sDeGybfD4gMybP4JKHotH8RrrHTm",
                    "stake": "701462142927258686063532903635",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "namdokmai.pool.f863973.m0",
                    "public_key": "ed25519:9uGeeM7j1fimpG7vn6EMcBXMei8ttWCohiMf44SoTzaz",
                    "stake": "697804708099777540795664741008",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "freshtest.pool.f863973.m0",
                    "public_key": "ed25519:5cbAt8uzmRztXWXKUYivtLsT2kMC414oHYDapfSJcgwv",
                    "stake": "695456947362945462887248283173",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "optimusvalidatornetwork.pool.f863973.m0",
                    "public_key": "ed25519:BGoxGmpvN7HdUSREQXfjH6kw5G6ph7NBXVfBVfUSH85V",
                    "stake": "659650194890927986127438929798",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "baziliknear.pool.f863973.m0",
                    "public_key": "ed25519:9Rbzfkhkk6RSa1HoPnJXS4q2nn1DwYeB4HMfJBB4WQpU",
                    "stake": "649640770859375849616470779507",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "blockscope.pool.f863973.m0",
                    "public_key": "ed25519:6K6xRp88BCQX5pcyrfkXDU371awMAmdXQY4gsxgjKmZz",
                    "stake": "648100336148549691745614006026",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "tagard.pool.f863973.m0",
                    "public_key": "ed25519:3KyziFgx3PpzorJnMFifXU4KsK4nwPFaxCGWTHaFBADK",
                    "stake": "645286495734589021006506438910",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "leadnode.pool.f863973.m0",
                    "public_key": "ed25519:CdP6CBFETfWYzrEedmpeqkR6rsJNeT22oUFn2mEDGk5i",
                    "stake": "642874001693431962382310806480",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "stakesstone.pool.f863973.m0",
                    "public_key": "ed25519:3aAdsKUuzZbjW9hHnmLWFRKwXjmcxsnLNLfNL4gP1wJ8",
                    "stake": "639687146467634324992676137322",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "basilisk-stake.pool.f863973.m0",
                    "public_key": "ed25519:CFo8vxoEUZoxbs87mGtG8qWUvSBHB91Vc6qWsaEXQ5cY",
                    "stake": "638435126612347314706093627131",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "shardlabs.pool.f863973.m0",
                    "public_key": "ed25519:DxmhGQZ6oqdxw7qGBvzLuBzE6XQjEh67hk5tt66vhLqL",
                    "stake": "636077017787496387778358128478",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "al3c5.pool.f863973.m0",
                    "public_key": "ed25519:BoYixTjyBePQ1VYP3s29rZfjtz1FLQ9og4FWZB5UgWCZ",
                    "stake": "635179037756388578983610676221",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "dehashed.pool.f863973.m0",
                    "public_key": "ed25519:EmPyD1DV9ajWJxjNN8GGACMyhM9w14brwNwYA5WvVaw",
                    "stake": "633751570338078700641701094115",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "projecttent.pool.f863973.m0",
                    "public_key": "ed25519:2ueHfYVewchegMmae9bc86ngdD1FWTbxewVb8sr4cABx",
                    "stake": "632964272403337126061579421240",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "machfund.pool.f863973.m0",
                    "public_key": "ed25519:G6fJ79oM6taQGhHeQZrg8N36nkCPMEVPyQMHfFT2wAKc",
                    "stake": "632916147599749298650761288493",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "blockngine.pool.f863973.m0",
                    "public_key": "ed25519:CZrTtCP6XkkxWtr3ATnXE8FL6bcG5cHcxfmdRgN7Lm7m",
                    "stake": "632276350961185290078490252819",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "grassets.pool.f863973.m0",
                    "public_key": "ed25519:3S4967Dt1VeeKrwBdTTR5tFEUFSwh17hEFLATRmtUNYV",
                    "stake": "621079903246623597023214134587",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "bflame.pool.f863973.m0",
                    "public_key": "ed25519:4uYM5RXgR9D6VAGKHgQTVNLEmCgMVX7PzpBstT92Me6R",
                    "stake": "615510378965857060709369749389",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "p0s.pool.f863973.m0",
                    "public_key": "ed25519:B4YpQ7qtD9w6VwujjJmZW8yrN5U13S5xuiTRiK63EzuF",
                    "stake": "614976797572607603450690973277",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "shurik.pool.f863973.m0",
                    "public_key": "ed25519:9zEn7DVpvQDxWdj5jSgrqJzqsLo8T9Wv37t83NXBiWi6",
                    "stake": "614898036349566235892974790043",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "dsrvlabs.pool.f863973.m0",
                    "public_key": "ed25519:61ei2efmmLkeDR1CG6JDEC2U3oZCUuC2K1X16Vmxrud9",
                    "stake": "612369210068271935619003639954",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "zetsi.pool.f863973.m0",
                    "public_key": "ed25519:6rYx5w1Z2pw46NBHv6Wo4JEUMNtqnDGqPaHT4wm15YRw",
                    "stake": "610463722294131289225382792740",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "n0ok.pool.f863973.m0",
                    "public_key": "ed25519:D6Gq2RpUoDUojmE2vLpqQzuZwYmFPW6rMcXPrwRYhqN8",
                    "stake": "592901786232128122037743373436",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "chelovek_iz_naroda.pool.f863973.m0",
                    "public_key": "ed25519:89aWsXXytjAZxyefXuGN73efnM9ugKTjPEGV4hDco8AZ",
                    "stake": "591365700020859570372350844871",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lavenderfive.pool.f863973.m0",
                    "public_key": "ed25519:AzwAiLDqprZKpDjhsH7dfyvFdfSasmPTjuJUAHfX1Pg4",
                    "stake": "584822141062242104503054368069",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "phet90testnet.pool.f863973.m0",
                    "public_key": "ed25519:AVaLksnE1S1A3mC6Mr3t9KnD67aA2R2vw68qTZ92MNu2",
                    "stake": "549661980218413341282459434513",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "everstake.pool.f863973.m0",
                    "public_key": "ed25519:4LDN8tZUTRRc4siGmYCPA67tRyxStACDchdGDZYKdFsw",
                    "stake": "545066659661515481050213108268",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "gullffa.pool.f863973.m0",
                    "public_key": "ed25519:79HUZcLERE4kLTraoaiEtJYCYeH6NZi6mYQ7YpbENazE",
                    "stake": "540374898559958822292290509248",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "infiniteloop.pool.f863973.m0",
                    "public_key": "ed25519:2fbiLqksH5viWXYoteyfKP9qQawkRKw4YogRFcvG3Z7f",
                    "stake": "537074035350708783248614238271",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "mintia.pool.f863973.m0",
                    "public_key": "ed25519:JAWDzHY7Ku99rW45WjS1Wh9fMc6CJ7M3vncnzoiTwfkL",
                    "stake": "513896135698482830743684944589",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lusienda.pool.f863973.m0",
                    "public_key": "ed25519:HdQb2HEiaMgvUdemTt5rkrFbxTpzZyELvg1Vov4LQAGU",
                    "stake": "507735395339414094713161711247",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "anchikovproduction.pool.f863973.m0",
                    "public_key": "ed25519:HDadu8UN6KTwenWdZRVmjsVnZhhKyLHLSNBYGCvrWmWg",
                    "stake": "502965984775268887396731241050",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "pool_easy2stake.pool.f863973.m0",
                    "public_key": "ed25519:8nzKxvmyeauQRehWkby8GfWNLgqPiF5FCRFSD75M1Rwh",
                    "stake": "481975311919861698673772620333",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "infstones.pool.f863973.m0",
                    "public_key": "ed25519:BLP6HB8tcwYRTxswQ2YRaJ5sGj1dgGpUUfcNwbnWFGCU",
                    "stake": "470740248039628426043694371910",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "prophet.pool.f863973.m0",
                    "public_key": "ed25519:HYJ9mUhxLhzSVtbjj89smAaZkMqXca68iCumZy3gySoB",
                    "stake": "353790651323396174147466683643",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "sashamaxymchuk.pool.f863973.m0",
                    "public_key": "ed25519:84G4fGj5nvuNq6WLqbBejApRjbRKztiWkqkLJ96gBwz7",
                    "stake": "152678736430094598759764034529",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "pennyvalidators.pool.f863973.m0",
                    "public_key": "ed25519:HiHdwq9rxi9hyxaGkazDHbYu4XL1j3J4TjgHQioyhEva",
                    "stake": "130869808574504390402978189179",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "kiln.pool.f863973.m0",
                    "public_key": "ed25519:Bq8fe1eUgDRexX2CYDMhMMQBiN13j8vTAVFyTNhEfh1W",
                    "stake": "96383628774902580179984175116",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "nodemeister.pool.f863973.m0",
                    "public_key": "ed25519:85EMyaNGMFuHK2RDH7KHno6fVYBR6iykUXHPPmFTGuTB",
                    "stake": "46912538187776826028199919919",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "nala.pool.f863973.m0",
                    "public_key": "ed25519:Fzwndob2h5PFdEuwo9eRFJV3BLLurcNaw2SGob5rMPEn",
                    "stake": "44463332518936197179505920782",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "happystake.pool.f863973.m0",
                    "public_key": "ed25519:3APqZiwzeZLzgfkJyGGTfepDYHA2d8NF1wZi4mCpZnaJ",
                    "stake": "43858080090365992839233826439",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "wolfedge-capital-testnet.pool.f863973.m0",
                    "public_key": "ed25519:CQEMcPQz6sqhAgoBm9ka9UeVcXj5NpNpRtDYYGkPggvg",
                    "stake": "37353127851533533932010335957",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "4ire-pool.pool.f863973.m0",
                    "public_key": "ed25519:EWPSvYN9pGPMmCLjVxx96stWdqksXNSGnfnuWYn9iiE5",
                    "stake": "33791378529794343090306270567",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lionstake.pool.f863973.m0",
                    "public_key": "ed25519:Fy6quR4nBhrEnDyEuPWoAdBP5tzNbuEZsEd91Q5pQnXB",
                    "stake": "33687600110478082848014759079",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "zentriav2.factory.colorpalette.testnet",
                    "public_key": "ed25519:4rCwSFzJ2e6suD5Yi7pgLidcAJ8Zt9BXieLzVedJDwmE",
                    "stake": "30425735139075156372196272474",
                    "validator_stake_struct_version": "V1"
                },
                {
                    "account_id": "lastnode.pool.f863973.m0",
                    "public_key": "ed25519:811gesxXYdYeThry96ZiWn8chgWYNyreiScMkmxg4U9u",
                    "stake": "24090352170251007644655353550",
                    "validator_stake_struct_version": "V1"
                }
            ],
            "prev_block_hash": "kobvwf6idnjzf1zUCdU8igL9G9ZUZyexkqVXFSpUVTK"
        },
        "id": "idontcare"
    }
    "#;

    #[test]
    fn test() {
        let client_block_view = get_client_block_view(CLIENT_BLOCK_RESPONSE).unwrap();

        assert_eq!(
            client_block_view.prev_block_hash.as_ref(),
            JSONCryptoHash::from(
                Base58CryptoHash::try_from("kobvwf6idnjzf1zUCdU8igL9G9ZUZyexkqVXFSpUVTK").unwrap()
            )
            .as_ref(),
        );

        assert_eq!(
            client_block_view.current_block_hash().as_ref(),
            JSONCryptoHash::from(
                Base58CryptoHash::try_from("DixB3qV9kRwPDWMKTuhBLM67QgW7bpJ6M5hrZr79kC8F").unwrap()
            )
            .as_ref(),
        )
    }
}
