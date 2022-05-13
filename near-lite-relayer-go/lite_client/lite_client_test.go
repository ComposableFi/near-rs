package lite_client

import (
	"crypto/sha256"
	"encoding/json"
	"log"
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/ComposableFi/near-trustless-bridge/near-lite-relayer-go/types"
	"github.com/btcsuite/btcutil/base58"
	"github.com/stretchr/testify/assert"
)

const PAYLOAD string = `{
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
}`

func getLightClientBlockView(t *testing.T, blockViewString string) (*types.LightClientBlockView, error) {
	type response struct {
		Result types.LightClientBlockViewJson `json:"result"`
	}

	var r response
	err := json.Unmarshal([]byte(blockViewString), &r)
	require.Nil(t, err)

	return r.Result.IntoLightClientBlockView()
}

func TestCalculateCurrentBlockHahs(t *testing.T) {
	lightClientBLockView, err := getLightClientBlockView(t, PAYLOAD)
	require.Nil(t, err)

	currentBlockHash, err := currentBlockHash(lightClientBLockView)
	require.Nil(t, err)

	assert.Equal(t, base58.Encode(currentBlockHash[:]), "DixB3qV9kRwPDWMKTuhBLM67QgW7bpJ6M5hrZr79kC8F")

}

func TestNextBlockHash(t *testing.T) {
	lightClientBLockView, err := getLightClientBlockView(t, PAYLOAD)
	require.Nil(t, err)

	currentBlockHash, err := currentBlockHash(lightClientBLockView)
	require.Nil(t, err)

	nextBlockHash, err := nextBlockHash(lightClientBLockView.NextBlockInnerHash, *currentBlockHash)
	require.Nil(t, err)

	assert.Equal(t, base58.Encode(nextBlockHash[:]), "HNfD1Kex1awMexrsjCUa8bUrykMecGUpysLv5dBTj5pK")
}

func TestApprovalMessage(t *testing.T) {

	lightClientBLockView, err := getLightClientBlockView(t, PAYLOAD)
	require.Nil(t, err)

	_, _, approvalMessage, err := reconstrunctLightClientBlockViewFields(lightClientBLockView)
	require.Nil(t, err)

	assert.Equal(t, base58.Encode(approvalMessage), "1D66k83oBABk1APcAcLQ1PAbXNixddhUJxhqWuGwTe8hLoxwsu8FJtgP")
}

func TestValidatorStakeViewSerialization(t *testing.T) {
	lightClientBLockViewPreviousBlock, err := getLightClientBlockView(t, CLIENT_RESPONSE_PREVIOUS_EPOCH)
	require.Nil(t, err)

	blockProducer := lightClientBLockViewPreviousBlock.NextBps[0]
	assert.Equal(t, "node1", blockProducer.V1.AccountId)
	assert.Equal(t, "ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su", base58.Encode(blockProducer.V1.PublicKey.ED25519.Inner[:]))
	assert.Equal(t, "22922510070824652286443844340832", blockProducer.V1.Stake.String())
}

const CLIENT_RESPONSE_PREVIOUS_EPOCH = `
{
	"jsonrpc": "2.0",
	"result": {
		"approvals_after_next": [
			"ed25519:4aQRJy2p92CYMc3EuRfM4oCHyobAL4VyL72e4n314ypQcUxZm7ynyCGh2Sb4kj3ESmEJeKxXZ6ejDcGhLd3UWqFc",
			null,
			"ed25519:4aQRJy2p92CYMc3EuRfM4oCHyobAL4VyL72e4n314ypQcUxZm7ynyCGh2Sb4kj3ESmEJeKxXZ6ejDcGhLd3UWqFc",
			null,
			"ed25519:4iaRL3pZfCjizdByKTxhBPYGc53UKQvN4Fe8S9RFJbvMcUFztJidwP4VtS9JNw8qWzu9Jt9mEe1XFRQwsDEm2jPT",
			"ed25519:geFMne98ZmAwJrVaNn81gX7K7yEuWUjPWeAyn3hvP1pwUN4t4BXbAguw4bCN8S2sxBoWmh8Yys4A63go2a6SeSN",
			"ed25519:5Kah5UWp685LL3eDKGcKr6XtEr8VzjWZZiNm46d2zuQSg3uG9fek6S9woiSMQqVZ3BA2MQDBNEbvYsBwDpmhsYYS",
			"ed25519:2Lv7M71AFYGSmc6deuBSSG8aC26qqyNjgV1i1zn2HEMDoi2D72JZgNPLc7HsV5hT18Z5s5fBp7zCAvSuxG94MV3S",
			"ed25519:4KGpWgCpoUQVbzfqc5kr2ZgMFeuM2einP6P3HMyiNi2QAk45oYNYHA88K9yhxsLqpxvqxo9UW6hYAsgDdNLjxG3g",
			"ed25519:ZBaanAehTj12JS49WsHoGfF2Koyo9sDad31mdKfSb1akXELSfHR1dJobH5EzYLxtH8njFqrakbVnSsy6WjNyLrk",
			"ed25519:4fjsB471GwjVfDzBZHpgurmZHe8aDazBav6BmGuSogoD5cd2u8B5k1qsC4EeKnphtKxfxZTUB8dHR88qd3KGogap",
			"ed25519:5FhdghVU5yxRybuaRg7g6ygi58dHXT4JbwXd4WA6UZnVcRnS6piePYGP5pn1c6xzMEVQVPXWKTGttXkeFwgprXwb",
			null,
			"ed25519:4SdYWFriHrku3c5zMHPBdJhfBpi3dvK5rGeRFy8c6jxychejj5qoCdSYhqqdqBDcBsXEJfJgfjo2RNBSc5Ap7zcF",
			null,
			"ed25519:2x1sRijVLZ49janrQgVF5dDCXBXb5QFHTySYnQ1VSCHHKJM3SxJLADbDazNBZbgTeh5frqpkzKAgvthpzoakXLdo",
			"ed25519:443peivFnX2QDNU5xdkRsQSK33AE1zBGjfELkpREGYDELeacfSYULG1kewQgEWBwBgDx6VSEewktjLHEeVEGqWbw",
			"ed25519:BBxXPu9z3gdvXfdbPLHgEVqE3G9TtPuCSn2MFa25tc1VZtxNByEEWUpj6sqfjVysdHgrRmN4tZY3nMBhznAApPV",
			"ed25519:3E9WDSv45wvqYDramYrTcBdDfsBEQBqEgFKxkR9L6oh2FtZnpyiMRLvXoEZkUPgsvmtueUXZbAYcrEc5cNTPx5Lr",
			"ed25519:2h4dRgT7VhMUsptSK4Qjj8Dhv1rqVAFfYEVHGMwSebc4mc5TviUtyp8PWKy4XwvAa18yBC3ePQU7zxaERPuTMZbJ",
			"ed25519:4jc1eJkqXActEwFSLBjcbqTqv15RWc5HnK8qHTJBRKXy4RSnB5DkhBmwWCnaz2gKVkqobmBpnE4ALZqDTKsTKLrH",
			null,
			"ed25519:KfXXkXvwNX3zzJ3WjBW1Rtg5pVdrq7xG1nuRmnErnAGms8zvavxUuoVXqxVRoipm6Toir57oAGuPZVxCqYmwHpB",
			"ed25519:3YjV8TNydcGxtBBwq9MxqbdLbzvaPhbFMqtho1a2AFSPPveNCp74QowoZhhjUYThaHMzpV5pW8GfbErjmgLoESmB",
			"ed25519:4rSsSHsXDE3HEwy7ACeiocRz7zT2cdUxVWjePyVjBueVEByTcZevFfGRWWG8LsJFC579vVYxMhWScgeGLvQUT6VU",
			"ed25519:JXUvESCXuJ4R6pWJcPQ191tvLcjGqQFv6bjYWEuNpepei49582hniphS9y2pNURx31LWtGZRNLJc6dpZpdCaNcF",
			"ed25519:1tqpLxMHba87vbjDg9oiAMrqkgUax8tHrYDMQQuYQx2YAeZ4QGPXAuh9R2XG54A3HfhKZizQid4Z7Q2PDoLF3CZ",
			"ed25519:2yktL6ZrmK2WPQYjWgGkubeCdbrLCrMvFcFxUMALwEymkdtbB4QFHUFRX5yZyqcZ1GUjBdyrMGAAajHA5khsTCRQ",
			"ed25519:4xfJQU4rdMGQKhS5aEvTa1AtSm4BPhPG4NgLqDCyYrMBwc8NPPJRdfVPNRHrUTxkXYwk5pVthngrWgxur64KVey2",
			"ed25519:iqShSEPpuQaAero3bq6qh9oCEZYpSVYD5Fe2gQwFLi39z36vhF77H5VsgRjcsx7EpvzSjgcNbwD2CWRBBrXTLi1",
			"ed25519:2xEAtUCqTZF8p9mPAj3nwkYLadWPB2wRZBbMaMhajBU7nuPbCZVVs3Ffo19L6BnLVWxRYdpx5qBwD3nhnprhERFP",
			"ed25519:4bv4NZ5FrwECgiowGGWAfQEC4JF2wHExN98aPwpEi5EeF5jZkFnAHv1ScXTUZPavzh8uXNBRpXLZCmJBsiLmKQzy",
			"ed25519:23SViFtn6fnAVSF5LiCyrYugpZpSaDWiMyahMuzxGyRgEJbv4js4BRLSixFL5gcDc7HzoA3D3fFjFRvW1XLiEyYb",
			"ed25519:HEiaox139rUMwt92L6RLvitqRSSqVhvv28cooZCQPVpCspz6vuYgWQF2o2jFbcW86AXybE2D4jBNwtJ7HQDYiFa",
			"ed25519:48o4rRJmqu1GJvkNBWUeCQ9UBiw4URfiakqGgvsas9aemK2gjHdUX3wePrPAEWRxnT3MPybJnsFbKBBRNuSXduTt",
			null,
			"ed25519:4XKAh8JrcvEnBfsLJvbUAjcPc8oR4Ep6fH1QvLyT85qvb9Z7DqkaEjbRJegcYVppSsHkEg25648TRHiFZPYi9yeK",
			"ed25519:ZDGgMkurGMnYsEcJKxqC6fBMfFfuGhNFDutomST4fHSRWHDwmUFZkhx74L9kAQkLvZAs2awTvf4FTWdcvvFnnbr",
			"ed25519:4pyZeRSH5WngPrKHs4SUYyziatDvpu2K97SC18BGFcSTHjzfE5evUQ8tnxWAUb17PjyyzuT8xhTqZrYDhNPyJBGR",
			"ed25519:k69YEdonZBgRBRVWnTnDTRLCGkPk7MPHdxystmPckCszGtyH9HzouE3xaZDWeSaY4zzYfVjKPq4j6kxFTa58fAq",
			null,
			"ed25519:xHxCLLN8J7JmTJVzctsaJMhSZjTkdwPAZSnYXNz76u2V1HPTBsvbw7zraYGK5wYFqtpEEo8M27xvy22pfWWWsVk",
			"ed25519:RdPGAPDp6eEWs7NnnHcQoNeAYaz2W93piweGkwfFv3kYgiaEYmX6tRmndVzqWoWrLJ1EarmmgAs6HQ4PFqPhxYn",
			"ed25519:37EebdaMgzYHSZghHJbMmbVVLzjMrYUyPYgD65YKm9LeKC8eYcqc9meJJgVfTYYufpzSHNftqt8wjrjtU9kwEo8n",
			"ed25519:pv3VngWFpHciNGF2rZvZEQhwXq83gYjhaRXetqwQgPfWbBa21WLjMHZB5jweVAt45Em6b7GzwYMRem2PaZXWQAT",
			"ed25519:33DPjeMKnQ367HuUpN2xgESQJhMidu84yuynLNPQLfmbFb8EBPMA1emqZx1r3t5YUvWS3ncvkrWzGuQkapWCbPW6",
			"ed25519:3YZNBgBzbM1fZKfnGCCfMdX6DxSgXXojTTr3q4P8T75P5iUcATQKKVVZcr8t5udWYsAzdi9Jj3S69L15HLDTT3X3",
			null,
			null,
			null,
			null,
			null,
			"ed25519:42bwC671pjEm9GP4mSdrTFtYaua8ThaETQGXK6o4EdBPo3PYfxufPkWhgduBaJ2UfACGxkb7tQTUyySKYAxmeiW7",
			null,
			null,
			null,
			null,
			null,
			null,
			"ed25519:46iybWtKT1Wk1Ads2isiD1Y2Qg6uKxngev16JPsjnX73kRWagpttLLw1ZahoYUQ5YyT159nnmHSzqHyTgkxaL6pB",
			"ed25519:3yn9MvYxWzXQkJmMX49dH1SLtKtK2vLHov5iEQgD4hAasH9BUcMV1ijvMnWjoWzPqk5UXa1DnwX2LNsT21YdqG6k",
			null,
			"ed25519:4N2tyS15xAtjtDiStx97PbJt8BbpEPYLJFwEuWpgP2M2foQWhyyHabPgWL9wNZmNobkPgVqgo5jQgWK61stMavN9",
			"ed25519:zpByZEzShynMRqDHkthxGSXeAsi28Dt9VBMrUqSSEKxuAqCToJKQGeiPNyFB28xpMbUnwsPiaBJJiCUxRBjjWm9",
			"ed25519:cZU7ShgEgFnKhNy9Cxq9dz3nC6vbZRWYSudyGXu2b5EgDmVTkvVXcSeiyJD1pJqUuRH84LDWBKbBkSG89B9xnHu",
			"ed25519:4idcFJZUq2z27H9AtAUKX9LmoSH6XRrMiEonCcQHzBNkBoJ34UnBQLDpYvQMe6S9QHWgKeq5DHtqWUeeheAg2LDs",
			"ed25519:4hveFV1Emah81wjDEoRNXgmUiFuEHMw58UqRHPkWEe41ResC1ceUynkJJEqz42XfdJEroqP8yMUo8YxmAEnKx4Te",
			"ed25519:44EaZSwnoqKYeccSsuut3oincFu1RWiCwbGhpxiiWj6TecZtvaoCwtq9sMYnw6iCbLfD2L6MtwQX9mUPJcGMnMrp",
			null,
			null,
			null,
			null,
			null,
			null
		],
		"inner_lite": {
			"block_merkle_root": "DC3LrxVzqdthS9jGFrL3fNAHPkPtWxDyAw6UjcGhMz13",
			"epoch_id": "5iyA2nJxvV4CVJwmfkK72X2f8s57shB2E174vWAPwHB7",
			"height": 86441383,
			"next_bp_hash": "8Uak4kmtmEmC9EN6TFJkhGd6UHHN9NSoJVZYZhJhf4hX",
			"next_epoch_id": "GHmqgUX59irTdh31mtuEs3uEaPNBY5sQTZjEX5w7ASgW",
			"outcome_root": "2AmR8gycvFzCp9sffcdrAW8RzBiVDwtL8sUjXA8MHMT3",
			"prev_state_root": "FkH8tWA59SEGKDwZSWtceR2GfCbyXHFQCHKXXgcPRqNb",
			"timestamp": 1648794682287664503,
			"timestamp_nanosec": "1648794682287664503"
		},
		"inner_rest_hash": "H7YRMumnCoyRVtthcU4UJLaZeorFbvKAKspbHbK2pzjf",
		"next_block_inner_hash": "CbrBCRpefTYNYfsgnhiDH76J4LBF4bZTRhMYg2FhtRrg",
		"next_bps": [
			{
				"account_id": "node1",
				"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
				"stake": "22922510070824652286443844340832",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "node0",
				"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
				"stake": "16925122454732557817312342323673",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "node2",
				"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
				"stake": "16874501568381514412356471157535",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "node3",
				"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
				"stake": "8567814429874820736296779398515",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "legends.pool.f863973.m0",
				"public_key": "ed25519:AhQ6sUifJYgjqarXSAzdDZU9ZixpUesP9JEH1Vr7NbaF",
				"stake": "5786557069698344213896712425679",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "staked.pool.f863973.m0",
				"public_key": "ed25519:D2afKYVaKQ1LGiWbMAZRfkKLgqimTR74wvtESvjx5Ft2",
				"stake": "4555135305460764820741929416211",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "masternode24.pool.f863973.m0",
				"public_key": "ed25519:9E3JvrQN6VGDGg1WJ3TjBsNyfmrU6kncBcDvvJLj6qHr",
				"stake": "3412581904036190940565941993636",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "01node.pool.f863973.m0",
				"public_key": "ed25519:3iNqnvBgxJPXCxu6hNdvJso1PEAc1miAD35KQMBCA3aL",
				"stake": "3057699516361921995426438471613",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "p2p.pool.f863973.m0",
				"public_key": "ed25519:4ie5979JdSR4f7MRAG58eghRxndVoKnAYAKa1PLoMYSS",
				"stake": "2954970535640565975430641632183",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "nodeasy.pool.f863973.m0",
				"public_key": "ed25519:25Dhg8NBvQhsVTuugav3t1To1X1zKiomDmnh8yN9hHMb",
				"stake": "1573328151521226380938038278779",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "tribe-pool.pool.f863973.m0",
				"public_key": "ed25519:CRS4HTSAeiP8FKD3c3ZrCL5pC92Mu1LQaWj22keThwFY",
				"stake": "1427529115848797786690040790199",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "chorusone.pool.f863973.m0",
				"public_key": "ed25519:3TkUuDpzrq75KtJhkuLfNNJBPHR5QEWpDxrter3znwto",
				"stake": "1277333300998711575968735209879",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "hotones.pool.f863973.m0",
				"public_key": "ed25519:2fc5xtbafKiLtxHskoPL2x7BpijxSZcwcAjzXceaxxWt",
				"stake": "1272041758046518449192873006540",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "foundryusa.pool.f863973.m0",
				"public_key": "ed25519:ABGnMW8c87ZKWxvZLLWgvrNe72HN7UoSf4cTBxCHbEE5",
				"stake": "1254473966505567163739276072923",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lunanova2.pool.f863973.m0",
				"public_key": "ed25519:9Jv6e9Kye4wM9EL1XJvXY8CYsLi1HLdRKnTzXBQY44w9",
				"stake": "1245973800630588895851102069596",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "chorus-one.pool.f863973.m0",
				"public_key": "ed25519:6LFwyEEsqhuDxorWfsKcPPs324zLWTaoqk4o6RDXN7Qc",
				"stake": "1109131454872548236874586501001",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "ni.pool.f863973.m0",
				"public_key": "ed25519:GfCfFkLk2twbAWdsS3tr7C2eaiHN3znSfbshS5e8NqBS",
				"stake": "1075644847325960842995579102016",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "cryptogarik.pool.f863973.m0",
				"public_key": "ed25519:FyFYc2MVwgitVf4NDLawxVoiwUZ1gYsxGesGPvaZcv6j",
				"stake": "839670626002142575910140045408",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "pathrocknetwork.pool.f863973.m0",
				"public_key": "ed25519:CGzLGZEMb84nRSRZ7Au1ETAoQyN7SQXQi55fYafXq736",
				"stake": "748664174850316067955626836603",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "stakely_v2.pool.f863973.m0",
				"public_key": "ed25519:7BanKZKGvFjK5Yy83gfJ71vPhqRwsDDyVHrV2FMJCUWr",
				"stake": "733920838342885843756234789662",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "solidstate.pool.f863973.m0",
				"public_key": "ed25519:DTDhqoMXDWhKedWpH7DPvR6dPDcXrk5pTHJw2bkFFvQy",
				"stake": "714369900526852891761727522625",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "aurora.pool.f863973.m0",
				"public_key": "ed25519:9c7mczZpNzJz98V1sDeGybfD4gMybP4JKHotH8RrrHTm",
				"stake": "702270431224324013411083156419",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "namdokmai.pool.f863973.m0",
				"public_key": "ed25519:9uGeeM7j1fimpG7vn6EMcBXMei8ttWCohiMf44SoTzaz",
				"stake": "698608807949059141913608464868",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "freshtest.pool.f863973.m0",
				"public_key": "ed25519:5cbAt8uzmRztXWXKUYivtLsT2kMC414oHYDapfSJcgwv",
				"stake": "696258382452410785529813203905",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "optimusvalidatornetwork.pool.f863973.m0",
				"public_key": "ed25519:BGoxGmpvN7HdUSREQXfjH6kw5G6ph7NBXVfBVfUSH85V",
				"stake": "660410303330680943777398509110",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "baziliknear.pool.f863973.m0",
				"public_key": "ed25519:9Rbzfkhkk6RSa1HoPnJXS4q2nn1DwYeB4HMfJBB4WQpU",
				"stake": "650389309598466011397359969840",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "blockscope.pool.f863973.m0",
				"public_key": "ed25519:6K6xRp88BCQX5pcyrfkXDU371awMAmdXQY4gsxgjKmZz",
				"stake": "648847313919662030580998874408",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "tagard.pool.f863973.m0",
				"public_key": "ed25519:3KyziFgx3PpzorJnMFifXU4KsK4nwPFaxCGWTHaFBADK",
				"stake": "646030292693079789339891316676",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "leadnode.pool.f863973.m0",
				"public_key": "ed25519:CdP6CBFETfWYzrEedmpeqkR6rsJNeT22oUFn2mEDGk5i",
				"stake": "643614800541578264591193039953",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "stakesstone.pool.f863973.m0",
				"public_key": "ed25519:3aAdsKUuzZbjW9hHnmLWFRKwXjmcxsnLNLfNL4gP1wJ8",
				"stake": "640424273252137206131666996427",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "basilisk-stake.pool.f863973.m0",
				"public_key": "ed25519:CFo8vxoEUZoxbs87mGtG8qWUvSBHB91Vc6qWsaEXQ5cY",
				"stake": "639170811025048237433711633196",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "shardlabs.pool.f863973.m0",
				"public_key": "ed25519:DxmhGQZ6oqdxw7qGBvzLuBzE6XQjEh67hk5tt66vhLqL",
				"stake": "636859864974473764754276519109",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "al3c5.pool.f863973.m0",
				"public_key": "ed25519:BoYixTjyBePQ1VYP3s29rZfjtz1FLQ9og4FWZB5UgWCZ",
				"stake": "635910968806069871670455050245",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "dehashed.pool.f863973.m0",
				"public_key": "ed25519:EmPyD1DV9ajWJxjNN8GGACMyhM9w14brwNwYA5WvVaw",
				"stake": "634481857193800599750766324892",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "machfund.pool.f863973.m0",
				"public_key": "ed25519:G6fJ79oM6taQGhHeQZrg8N36nkCPMEVPyQMHfFT2wAKc",
				"stake": "633945472280870168216616557188",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "projecttent.pool.f863973.m0",
				"public_key": "ed25519:2ueHfYVewchegMmae9bc86ngdD1FWTbxewVb8sr4cABx",
				"stake": "633694652515862591275483076651",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "blockngine.pool.f863973.m0",
				"public_key": "ed25519:CZrTtCP6XkkxWtr3ATnXE8FL6bcG5cHcxfmdRgN7Lm7m",
				"stake": "633004909477506166031191906205",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "grassets.pool.f863973.m0",
				"public_key": "ed25519:3S4967Dt1VeeKrwBdTTR5tFEUFSwh17hEFLATRmtUNYV",
				"stake": "621795588696433313338045066893",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "bflame.pool.f863973.m0",
				"public_key": "ed25519:4uYM5RXgR9D6VAGKHgQTVNLEmCgMVX7PzpBstT92Me6R",
				"stake": "616319592025058485087576215014",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "shurik.pool.f863973.m0",
				"public_key": "ed25519:9zEn7DVpvQDxWdj5jSgrqJzqsLo8T9Wv37t83NXBiWi6",
				"stake": "615607598217041309240812408441",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "p0s.pool.f863973.m0",
				"public_key": "ed25519:B4YpQ7qtD9w6VwujjJmZW8yrN5U13S5xuiTRiK63EzuF",
				"stake": "614976797913850275582890973277",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "dsrvlabs.pool.f863973.m0",
				"public_key": "ed25519:61ei2efmmLkeDR1CG6JDEC2U3oZCUuC2K1X16Vmxrud9",
				"stake": "613074857526823170190346894417",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "zetsi.pool.f863973.m0",
				"public_key": "ed25519:6rYx5w1Z2pw46NBHv6Wo4JEUMNtqnDGqPaHT4wm15YRw",
				"stake": "611167150967495369878691581128",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "n0ok.pool.f863973.m0",
				"public_key": "ed25519:D6Gq2RpUoDUojmE2vLpqQzuZwYmFPW6rMcXPrwRYhqN8",
				"stake": "593639947175383273636863070076",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "chelovek_iz_naroda.pool.f863973.m0",
				"public_key": "ed25519:89aWsXXytjAZxyefXuGN73efnM9ugKTjPEGV4hDco8AZ",
				"stake": "592047145048110825510136899314",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lavenderfive.pool.f863973.m0",
				"public_key": "ed25519:AzwAiLDqprZKpDjhsH7dfyvFdfSasmPTjuJUAHfX1Pg4",
				"stake": "585546022984649856687381527767",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "latenthero.pool.f863973.m0",
				"public_key": "ed25519:EQqmjRNouRKhwGL7Hnp3vcbDywg2Boj6to2gmnXybhEM",
				"stake": "579758122709566871952544545842",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "smcvalidator.pool.f863973.m0",
				"public_key": "ed25519:pG4LYsyoAa8yWYG9nsTQ5yBcwke51i3VqeRcMVbE9Q7",
				"stake": "555422197586970576403131175346",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "phet90testnet.pool.f863973.m0",
				"public_key": "ed25519:AVaLksnE1S1A3mC6Mr3t9KnD67aA2R2vw68qTZ92MNu2",
				"stake": "549661980894401124487359434513",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "everstake.pool.f863973.m0",
				"public_key": "ed25519:4LDN8tZUTRRc4siGmYCPA67tRyxStACDchdGDZYKdFsw",
				"stake": "545692905175646615423583560444",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "rossi-validator.pool.f863973.m0",
				"public_key": "ed25519:2eRx2c3KX9wFd3EzuuajFQoSxRTKDqSbxcF13LfkrxCR",
				"stake": "545396693341301216467002952473",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "gullffa.pool.f863973.m0",
				"public_key": "ed25519:79HUZcLERE4kLTraoaiEtJYCYeH6NZi6mYQ7YpbENazE",
				"stake": "540374898942935788531590509248",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "infiniteloop.pool.f863973.m0",
				"public_key": "ed25519:2fbiLqksH5viWXYoteyfKP9qQawkRKw4YogRFcvG3Z7f",
				"stake": "537692918603524974448590839751",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "mintia.pool.f863973.m0",
				"public_key": "ed25519:JAWDzHY7Ku99rW45WjS1Wh9fMc6CJ7M3vncnzoiTwfkL",
				"stake": "513896136137419546135584944589",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lusienda.pool.f863973.m0",
				"public_key": "ed25519:HdQb2HEiaMgvUdemTt5rkrFbxTpzZyELvg1Vov4LQAGU",
				"stake": "508420470175664819906389753582",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "anchikovproduction.pool.f863973.m0",
				"public_key": "ed25519:HDadu8UN6KTwenWdZRVmjsVnZhhKyLHLSNBYGCvrWmWg",
				"stake": "502965985183139930884631241050",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "ino.pool.f863973.m0",
				"public_key": "ed25519:B75h2eqpaMgh6WkAvgnz2FsEC9s5TwVx7zwTjqXKfRs6",
				"stake": "494974817176910920780821071715",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "pontiff.pool.f863973.m0",
				"public_key": "ed25519:4i8j7nwNyy18hfARtrVpckT8MiicdCXuWBX1TubdMb5Y",
				"stake": "478587210879643963063840990682",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "prophet.pool.f863973.m0",
				"public_key": "ed25519:HYJ9mUhxLhzSVtbjj89smAaZkMqXca68iCumZy3gySoB",
				"stake": "353790651507020866141066683643",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "sashamaxymchuk.pool.f863973.m0",
				"public_key": "ed25519:84G4fGj5nvuNq6WLqbBejApRjbRKztiWkqkLJ96gBwz7",
				"stake": "152678736698423437298364034529",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "kiln.pool.f863973.m0",
				"public_key": "ed25519:Bq8fe1eUgDRexX2CYDMhMMQBiN13j8vTAVFyTNhEfh1W",
				"stake": "96495618271143227175107926458",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "nodemeister.pool.f863973.m0",
				"public_key": "ed25519:85EMyaNGMFuHK2RDH7KHno6fVYBR6iykUXHPPmFTGuTB",
				"stake": "46966596609590542375097063730",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "nala.pool.f863973.m0",
				"public_key": "ed25519:Fzwndob2h5PFdEuwo9eRFJV3BLLurcNaw2SGob5rMPEn",
				"stake": "44714508225885118642457972042",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "happystake.pool.f863973.m0",
				"public_key": "ed25519:3APqZiwzeZLzgfkJyGGTfepDYHA2d8NF1wZi4mCpZnaJ",
				"stake": "43908619030797948815749119844",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "wolfedge-capital-testnet.pool.f863973.m0",
				"public_key": "ed25519:CQEMcPQz6sqhAgoBm9ka9UeVcXj5NpNpRtDYYGkPggvg",
				"stake": "37421154012756824118554142900",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "jstaking.pool.f863973.m0",
				"public_key": "ed25519:fui1E5XwnAWGYDBSQ3168aDfsW1KDFH8A7nBHvZiqGv",
				"stake": "36368375187772860724451257216",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "4ire-pool.pool.f863973.m0",
				"public_key": "ed25519:EWPSvYN9pGPMmCLjVxx96stWdqksXNSGnfnuWYn9iiE5",
				"stake": "33830317130679015785634403282",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lionstake.pool.f863973.m0",
				"public_key": "ed25519:Fy6quR4nBhrEnDyEuPWoAdBP5tzNbuEZsEd91Q5pQnXB",
				"stake": "33726419125235534318928178338",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "zentriav2.factory.colorpalette.testnet",
				"public_key": "ed25519:4rCwSFzJ2e6suD5Yi7pgLidcAJ8Zt9BXieLzVedJDwmE",
				"stake": "30560796702897684567471814711",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lastnode.pool.f863973.m0",
				"public_key": "ed25519:811gesxXYdYeThry96ZiWn8chgWYNyreiScMkmxg4U9u",
				"stake": "24118112236988428679747661113",
				"validator_stake_struct_version": "V1"
			}
		],
		"prev_block_hash": "FD128fQ4vBeCKqEnkfCGdbVWXZPcaCfrAY3MSpib1mDr"
	},
	"id": "idontcare"
}`

// Block #86455884
const CLIENT_BLOCK_RESPONSE = `
{
	"jsonrpc": "2.0",
	"result": {
		"approvals_after_next": [
			"ed25519:26AdvKhPjpJSvednVPzvKzzauvEixBHaatmjR5P1jNYKPShgdi6BgMFUrebGbS4aAAA7CbE8JamcJ13SoKBKYQEX",
			null,
			"ed25519:26AdvKhPjpJSvednVPzvKzzauvEixBHaatmjR5P1jNYKPShgdi6BgMFUrebGbS4aAAA7CbE8JamcJ13SoKBKYQEX",
			"ed25519:26AdvKhPjpJSvednVPzvKzzauvEixBHaatmjR5P1jNYKPShgdi6BgMFUrebGbS4aAAA7CbE8JamcJ13SoKBKYQEX",
			"ed25519:38zDqDYHaW35Ag9U1YZudiMdUZBKtQqKgo8S7rKWP6Kt2dx6RDbBZDSYTeojhY6WkdKaG7tdPXZgJFEvRU4TjMsS",
			"ed25519:5zApVpNrJ1xhqtBACN6ZpR6ePcrKAHvvKfKpx365n87XXWTLj8MtGZvkZ6netaX9wesHPejmABJwfTvMCcv8ofay",
			"ed25519:42gdZewdzK3EHY4zre69rVTMU2yuwpFd13tDv4JopPpqE7x218xDGmtThrii3r7AVLMJc49TvuueUc87JKJKi1g9",
			"ed25519:EEXpy5RLuCBHe8XWQio8QjzaJTRb7RmJCtubwaNUGm1tja2zF2YgEfpVQx2ywsNCtsJvGqMGW6JWaDZLnErCFVJ",
			"ed25519:3EhiCzKkmcviyhdf7rfjhSZjm8soxp2DyZj41ryhHJ8FpW7QXU7VmBpeDbN1zG8AuAzbSxeHTRVLAhegnQVtbb5J",
			"ed25519:3uU4J5dc7VZpNqwB1CXaFpCzAzxiAcZNF73jEwcGHrT7bqFZ2mZtFGKmHZxEnDVJdPjQNXrgXePaWSZdvpiXWf7G",
			"ed25519:2SvJzrJZhabvpuwTmGXAxkjJvjKEVWVp3Q63k3u5AyDbbZZTWFU6UDyTJt1YLKmuSFD9p6t4BUWE5E3S88mvmzvG",
			"ed25519:24vdiR1GuDShms851Pd2LreemL9NRqacXzfbHN2zH7iNoSQ4sVz5QhhDaH6kuibRFMT1cqok8zU6o6PNWXbkrkia",
			null,
			"ed25519:5dw8MnX6xt6QnnDhSfjaJAo4gHWb2QSLacLcgGQoUwRq12E5inzfcJgkG742LbFq5xKhvZCshKiNvTPmK5oZWnny",
			null,
			"ed25519:2HXALBATYbkj35ioUqmRtBU7wxcc8CHiNtZS8wk7c95Y229bqP8wznK8RG53dLYXcm9647BRh9SFXMt2eFnmNB3B",
			"ed25519:3VKHEkpXNCWR5jjsuVDfMCdxbYjtcqmWqW5823HDrgyWG7mr1cWZActykTvRR56EDbDsFfDLU1cjHGgrg1yrbJpX",
			null,
			"ed25519:UWFcxWgoq3EnKQV7sfmmCeg56jdhVAD7enpE1mVQkRsRc29cp8FQSeuMw5PgfdvLQDsBvh3jE9e7mHbZdUhwugF",
			null,
			null,
			null,
			null,
			"ed25519:3X2DCE39PMbXAkPSUUv26RuvrkMD4mgxFer4Vx1NyRu5G8gaMmDF5anASkWwAUranoJgcdb17iHiVqGXKi8j3kww",
			null,
			"ed25519:3BY1dzwxJ3jyPVcSMnRYm2NLytV3Hn7WDPwrwjwotkTzM3bQbj5soXQjtfpChWkoExAvF8nRteLQ7xCiiGjTNyWs",
			"ed25519:473YNvCkBZSDX3G4x8hvcpnEtogKhxBnLYa5pGykKourLcR3vKZCeUQ19SEfMk1K1qreUL692N1qicN8FtCG745n",
			null,
			"ed25519:5E1TJ2akcffmbTFb2LbDpScdqicyck8WKsm9gZZs9vgPgo2HxLxd9JJib79TqgDWRB9WUiiB2cxmDXBiPwXbjyAp",
			null,
			"ed25519:5qAHAoYpVB86gWBfakLxJYD1aSetXUfmass4Q4hiXiJ6KZgktzFqVaxUjYyBwtpjcqQrtGbsLpjkRz4orjENkJiv",
			null,
			"ed25519:5M2J4oDYi798Zzsp5KyusUYTaGeTZPEZAPjoPUUDkRLZDu4omMnVMArJABu7geFGGJ24RvcuMFvxxmgLFjuhyEkx",
			"ed25519:5wG7q6qucqDMZmDq8KYdisSUuGxMc795c7KoM2BMT2J8LRoQLsLESjZRxk8CEF3GdzHdhfy5NFaopRuNLoePzksz",
			null,
			null,
			null,
			"ed25519:5QatrWTnrjdCiXmiG95gAhvyeggaXwCzHVmTQQJ5YRTejuJRjkiKJJ6VKHQG1ENfWkLFce4ho4sP9U5xjd3z82m9",
			null,
			"ed25519:5cByB2PH8qzWJcqvx4sTPi59kC6iNk2mWLo3aBuAUY8c5kGZu1Co3q9KVxLdnwBhdjw6RDFuefqWtqHmENP2HnMb",
			null,
			null,
			"ed25519:53euGtGneS9pMRfoak8TwpKnFxngWXEs11GStuhkrZAKgWAGFcc6FMtPHrLS6h9jZvYZ1DRArTy1RRuwcVENuBMA",
			"ed25519:3tSn2cFRcKPfRNx4ayC8JUsV2FV3ZRuCUe4FUL71HFmDoE9TXzEZbLnPCEDm4fDYAibHs6AQGZeAqGBKvodNZLj5",
			"ed25519:Nz1xPY9VFdtidAP3rBmdRbn3g8gBvP5a3eTik9GyBmaNfiJWqg8GFM2nVHa4o9rjtenmbhTSrQszX6XGoKDcsGU",
			null,
			null,
			null,
			null,
			null,
			null,
			null,
			null,
			null,
			"ed25519:5HTWzh8SBSnJwPrPUuog7DCu5KnnPibvt2bAY8acC9XwGTy3n2kbJViwXvgpFmcoPFpyNSTR2uA6v5sqb1URAdEz",
			null,
			null,
			null,
			null,
			null,
			"ed25519:3dE44DTNBcJjvQ7ZZcXCUnqnWN1Ut4swxRqzv1frdzFN2MYsNBx7rMmmiXGr33zJ7jXWRTLbJgBHG8b9Tsy8kdnZ",
			"ed25519:4R8wrJjPinQAeyXxHHZuu25nsQVdw5RcN8fF1hzaGHTycjonj1WgeAJzm88WJNcgjMbbRgn8BJzR4bQCF3UfN7Pr",
			null,
			"ed25519:5aLBAW42sdqJhQe51jvw21XACgBXgSV2R4bBFjc8LpJBhSFu6USoNzNkAEcrtH4fiFC5eU8mh2cNWJYRS7eBabi9",
			"ed25519:2iW39NZjxGq6XU7miQWmDsMNnbVXKu48P87PJoX2sDTz74GegGcoD71eX8Fm8K9umyARFffFU1CKXyA1eoFnefs1",
			null,
			"ed25519:3kNinyfW8exUi8Qi5nr1MX1DB27yDEfjnmq4hsAVXbACFgBfcJgLAysSp7Ue38URQtwEVnt9pefVbZ8PPP6wDKGz",
			"ed25519:5X6Fq8PeNtc6sv84QeKPd5MG4La9K3rBMDYbKtkJ8VZcC6k1ehFd9NP3PuBqwL5gMqoqj7nkzSQZzJzKDJLPJRCA",
			null,
			null
		],
		"inner_lite": {
			"block_merkle_root": "9RzsdrtaXvCup17nAfVdHwxTd76j1FNaW7e7z2McuZHs",
			"epoch_id": "GHmqgUX59irTdh31mtuEs3uEaPNBY5sQTZjEX5w7ASgW",
			"height": 86455909,
			"next_bp_hash": "9VPzyStHi4X2T7VAbfSTbLXEd8vjFP7wFJjYyjSJxQik",
			"next_epoch_id": "8nVTHDfxg2G8AWbKhVfFtnEb5jJeiXV2XBFsdyt2cif1",
			"outcome_root": "ceLGxBWvaytQzku8q3NoxuE3922MnMFxsNEVCLPBeUD",
			"prev_state_root": "AJuNNNFMaeKJc4Ufapuy3hc1jkoPLvFbVWpXcZdLfg1U",
			"timestamp": 1648810040622608566,
			"timestamp_nanosec": "1648810040622608566"
		},
		"inner_rest_hash": "6SK4tgTksZyLx9cW2neu4xBXuJPkaseFrR9riTKMLJwv",
		"next_block_inner_hash": "2fwovRgRy3GNv4nivi8PgwM3aJAeedbvWYMCDYt5CsYG",
		"next_bps": [
			{
				"account_id": "node1",
				"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
				"stake": "22949327592242450816363151898853",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "node0",
				"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
				"stake": "16944923507607057621836326590864",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "node2",
				"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
				"stake": "16894243398827941870356919783063",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "node3",
				"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
				"stake": "8577838094223400746241842212915",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "legends.pool.f863973.m0",
				"public_key": "ed25519:AhQ6sUifJYgjqarXSAzdDZU9ZixpUesP9JEH1Vr7NbaF",
				"stake": "5793326871499643941084500854531",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "staked.pool.f863973.m0",
				"public_key": "ed25519:D2afKYVaKQ1LGiWbMAZRfkKLgqimTR74wvtESvjx5Ft2",
				"stake": "4559762052294055739961541809028",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "masternode24.pool.f863973.m0",
				"public_key": "ed25519:9E3JvrQN6VGDGg1WJ3TjBsNyfmrU6kncBcDvvJLj6qHr",
				"stake": "3416574120678826701003147150326",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "01node.pool.f863973.m0",
				"public_key": "ed25519:3iNqnvBgxJPXCxu6hNdvJso1PEAc1miAD35KQMBCA3aL",
				"stake": "3061276782639300406837420592214",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "p2p.pool.f863973.m0",
				"public_key": "ed25519:4ie5979JdSR4f7MRAG58eghRxndVoKnAYAKa1PLoMYSS",
				"stake": "2958427611565856637171061933942",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "nodeasy.pool.f863973.m0",
				"public_key": "ed25519:25Dhg8NBvQhsVTuugav3t1To1X1zKiomDmnh8yN9hHMb",
				"stake": "1575068818350064235628643461649",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "tribe-pool.pool.f863973.m0",
				"public_key": "ed25519:CRS4HTSAeiP8FKD3c3ZrCL5pC92Mu1LQaWj22keThwFY",
				"stake": "1429199212043501677779067532132",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "chorusone.pool.f863973.m0",
				"public_key": "ed25519:3TkUuDpzrq75KtJhkuLfNNJBPHR5QEWpDxrter3znwto",
				"stake": "1278827676875609593894511486301",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "hotones.pool.f863973.m0",
				"public_key": "ed25519:2fc5xtbafKiLtxHskoPL2x7BpijxSZcwcAjzXceaxxWt",
				"stake": "1273529881837124230828073909315",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "foundryusa.pool.f863973.m0",
				"public_key": "ed25519:ABGnMW8c87ZKWxvZLLWgvrNe72HN7UoSf4cTBxCHbEE5",
				"stake": "1256081604638924285747937189845",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lunanova2.pool.f863973.m0",
				"public_key": "ed25519:9Jv6e9Kye4wM9EL1XJvXY8CYsLi1HLdRKnTzXBQY44w9",
				"stake": "1247431491303762172509349058430",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "chorus-one.pool.f863973.m0",
				"public_key": "ed25519:6LFwyEEsqhuDxorWfsKcPPs324zLWTaoqk4o6RDXN7Qc",
				"stake": "1110429050842727763339891353120",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "ni.pool.f863973.m0",
				"public_key": "ed25519:GfCfFkLk2twbAWdsS3tr7C2eaiHN3znSfbshS5e8NqBS",
				"stake": "1076903268858699791106964347506",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "cryptogarik.pool.f863973.m0",
				"public_key": "ed25519:FyFYc2MVwgitVf4NDLawxVoiwUZ1gYsxGesGPvaZcv6j",
				"stake": "840652974653901124214299092043",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "pathrocknetwork.pool.f863973.m0",
				"public_key": "ed25519:CGzLGZEMb84nRSRZ7Au1ETAoQyN7SQXQi55fYafXq736",
				"stake": "749739988926667488225409312930",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "stakely_v2.pool.f863973.m0",
				"public_key": "ed25519:7BanKZKGvFjK5Yy83gfJ71vPhqRwsDDyVHrV2FMJCUWr",
				"stake": "734779467803676488422251769143",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "solidstate.pool.f863973.m0",
				"public_key": "ed25519:DTDhqoMXDWhKedWpH7DPvR6dPDcXrk5pTHJw2bkFFvQy",
				"stake": "715205657993906057594050568659",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "aurora.pool.f863973.m0",
				"public_key": "ed25519:9c7mczZpNzJz98V1sDeGybfD4gMybP4JKHotH8RrrHTm",
				"stake": "703162032315675728652111978820",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "namdokmai.pool.f863973.m0",
				"public_key": "ed25519:9uGeeM7j1fimpG7vn6EMcBXMei8ttWCohiMf44SoTzaz",
				"stake": "699426128043696790256527911933",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "freshtest.pool.f863973.m0",
				"public_key": "ed25519:5cbAt8uzmRztXWXKUYivtLsT2kMC414oHYDapfSJcgwv",
				"stake": "697072950038835725218153979145",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "optimusvalidatornetwork.pool.f863973.m0",
				"public_key": "ed25519:BGoxGmpvN7HdUSREQXfjH6kw5G6ph7NBXVfBVfUSH85V",
				"stake": "661182931526239970852421432715",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "baziliknear.pool.f863973.m0",
				"public_key": "ed25519:9Rbzfkhkk6RSa1HoPnJXS4q2nn1DwYeB4HMfJBB4WQpU",
				"stake": "651150213650042597898598894903",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "blockscope.pool.f863973.m0",
				"public_key": "ed25519:6K6xRp88BCQX5pcyrfkXDU371awMAmdXQY4gsxgjKmZz",
				"stake": "649506414222131713576984442889",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "tagard.pool.f863973.m0",
				"public_key": "ed25519:3KyziFgx3PpzorJnMFifXU4KsK4nwPFaxCGWTHaFBADK",
				"stake": "646786097203475534304943885178",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "leadnode.pool.f863973.m0",
				"public_key": "ed25519:CdP6CBFETfWYzrEedmpeqkR6rsJNeT22oUFn2mEDGk5i",
				"stake": "644367778886663802105399198378",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "stakesstone.pool.f863973.m0",
				"public_key": "ed25519:3aAdsKUuzZbjW9hHnmLWFRKwXjmcxsnLNLfNL4gP1wJ8",
				"stake": "641198519157648602505664886163",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "basilisk-stake.pool.f863973.m0",
				"public_key": "ed25519:CFo8vxoEUZoxbs87mGtG8qWUvSBHB91Vc6qWsaEXQ5cY",
				"stake": "639918590440004706626411243128",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "shardlabs.pool.f863973.m0",
				"public_key": "ed25519:DxmhGQZ6oqdxw7qGBvzLuBzE6XQjEh67hk5tt66vhLqL",
				"stake": "637803882455578964186296090355",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "al3c5.pool.f863973.m0",
				"public_key": "ed25519:BoYixTjyBePQ1VYP3s29rZfjtz1FLQ9og4FWZB5UgWCZ",
				"stake": "636854880374440657378246667596",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "dehashed.pool.f863973.m0",
				"public_key": "ed25519:EmPyD1DV9ajWJxjNN8GGACMyhM9w14brwNwYA5WvVaw",
				"stake": "635224150718459403099965806552",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "machfund.pool.f863973.m0",
				"public_key": "ed25519:G6fJ79oM6taQGhHeQZrg8N36nkCPMEVPyQMHfFT2wAKc",
				"stake": "634686788251976758263963874506",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "blockngine.pool.f863973.m0",
				"public_key": "ed25519:CZrTtCP6XkkxWtr3ATnXE8FL6bcG5cHcxfmdRgN7Lm7m",
				"stake": "633656065475669726280826427959",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "grassets.pool.f863973.m0",
				"public_key": "ed25519:3S4967Dt1VeeKrwBdTTR5tFEUFSwh17hEFLATRmtUNYV",
				"stake": "622722987982775798532829252304",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "bflame.pool.f863973.m0",
				"public_key": "ed25519:4uYM5RXgR9D6VAGKHgQTVNLEmCgMVX7PzpBstT92Me6R",
				"stake": "617234461115345372278772960093",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "shurik.pool.f863973.m0",
				"public_key": "ed25519:9zEn7DVpvQDxWdj5jSgrqJzqsLo8T9Wv37t83NXBiWi6",
				"stake": "616327809807619407716759066614",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "dsrvlabs.pool.f863973.m0",
				"public_key": "ed25519:61ei2efmmLkeDR1CG6JDEC2U3oZCUuC2K1X16Vmxrud9",
				"stake": "613792106557214713239288385761",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "zetsi.pool.f863973.m0",
				"public_key": "ed25519:6rYx5w1Z2pw46NBHv6Wo4JEUMNtqnDGqPaHT4wm15YRw",
				"stake": "611882168159257611258042281605",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "n0ok.pool.f863973.m0",
				"public_key": "ed25519:D6Gq2RpUoDUojmE2vLpqQzuZwYmFPW6rMcXPrwRYhqN8",
				"stake": "594349395199079126466241101938",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "chelovek_iz_naroda.pool.f863973.m0",
				"public_key": "ed25519:89aWsXXytjAZxyefXuGN73efnM9ugKTjPEGV4hDco8AZ",
				"stake": "592739793772796190513231168872",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lavenderfive.pool.f863973.m0",
				"public_key": "ed25519:AzwAiLDqprZKpDjhsH7dfyvFdfSasmPTjuJUAHfX1Pg4",
				"stake": "586231008421809079867645695624",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "latenthero.pool.f863973.m0",
				"public_key": "ed25519:EQqmjRNouRKhwGL7Hnp3vcbDywg2Boj6to2gmnXybhEM",
				"stake": "579738101137715103577294987834",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "tayang.pool.f863973.m0",
				"public_key": "ed25519:G9XWX55MfWEpT84ckcsJxVTKeZK4WqBGJX3xVpnPB5vv",
				"stake": "563498889920635651950224126233",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "smcvalidator.pool.f863973.m0",
				"public_key": "ed25519:pG4LYsyoAa8yWYG9nsTQ5yBcwke51i3VqeRcMVbE9Q7",
				"stake": "555422197586970576403131175346",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "everstake.pool.f863973.m0",
				"public_key": "ed25519:4LDN8tZUTRRc4siGmYCPA67tRyxStACDchdGDZYKdFsw",
				"stake": "546400197607367519956748211889",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "rossi-validator.pool.f863973.m0",
				"public_key": "ed25519:2eRx2c3KX9wFd3EzuuajFQoSxRTKDqSbxcF13LfkrxCR",
				"stake": "545396693549586230215202952473",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "infiniteloop.pool.f863973.m0",
				"public_key": "ed25519:2fbiLqksH5viWXYoteyfKP9qQawkRKw4YogRFcvG3Z7f",
				"stake": "538321976932135835213436874121",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lusienda.pool.f863973.m0",
				"public_key": "ed25519:HdQb2HEiaMgvUdemTt5rkrFbxTpzZyELvg1Vov4LQAGU",
				"stake": "509015164869674763004419847436",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "ino.pool.f863973.m0",
				"public_key": "ed25519:B75h2eqpaMgh6WkAvgnz2FsEC9s5TwVx7zwTjqXKfRs6",
				"stake": "494974817444468749939621071716",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "pontiff.pool.f863973.m0",
				"public_key": "ed25519:4i8j7nwNyy18hfARtrVpckT8MiicdCXuWBX1TubdMb5Y",
				"stake": "478587210879643963063840990682",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "kiln.pool.f863973.m0",
				"public_key": "ed25519:Bq8fe1eUgDRexX2CYDMhMMQBiN13j8vTAVFyTNhEfh1W",
				"stake": "96608509421037438882028377566",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "nodemeister.pool.f863973.m0",
				"public_key": "ed25519:85EMyaNGMFuHK2RDH7KHno6fVYBR6iykUXHPPmFTGuTB",
				"stake": "47021543808070096585479049932",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "nala.pool.f863973.m0",
				"public_key": "ed25519:Fzwndob2h5PFdEuwo9eRFJV3BLLurcNaw2SGob5rMPEn",
				"stake": "44766587364445748049092546945",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "happystake.pool.f863973.m0",
				"public_key": "ed25519:3APqZiwzeZLzgfkJyGGTfepDYHA2d8NF1wZi4mCpZnaJ",
				"stake": "43959988855512773720415910025",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "ibb.pool.f863973.m0",
				"public_key": "ed25519:7gvdHhcMcXT1jMZoxDKy7yXnRiPVX1tAFTa7HWTHbe8C",
				"stake": "42001690004861681144621857517",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "mateennala.pool.f863973.m0",
				"public_key": "ed25519:9kNpQKUKzhc1AiFSEoZcTNapTnywjbXBPngH3EDpD1tw",
				"stake": "40056014128143748170300000000",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "wolfedge-capital-testnet.pool.f863973.m0",
				"public_key": "ed25519:CQEMcPQz6sqhAgoBm9ka9UeVcXj5NpNpRtDYYGkPggvg",
				"stake": "37464905110868615156797728096",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "jstaking.pool.f863973.m0",
				"public_key": "ed25519:fui1E5XwnAWGYDBSQ3168aDfsW1KDFH8A7nBHvZiqGv",
				"stake": "36368375383183646876651257216",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "dariya.pool.f863973.m0",
				"public_key": "ed25519:A5Rx38TsNKWXzF5o18HpaRrPeBzv3riqur51bqhU1Qbp",
				"stake": "36211347514033914937590010268",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "4ire-pool.pool.f863973.m0",
				"public_key": "ed25519:EWPSvYN9pGPMmCLjVxx96stWdqksXNSGnfnuWYn9iiE5",
				"stake": "33869896086305183386478534323",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lionstake.pool.f863973.m0",
				"public_key": "ed25519:Fy6quR4nBhrEnDyEuPWoAdBP5tzNbuEZsEd91Q5pQnXB",
				"stake": "33765876364623459491244697143",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "zentriav2.factory.colorpalette.testnet",
				"public_key": "ed25519:4rCwSFzJ2e6suD5Yi7pgLidcAJ8Zt9BXieLzVedJDwmE",
				"stake": "30596434283244809799848018489",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lastnode.pool.f863973.m0",
				"public_key": "ed25519:811gesxXYdYeThry96ZiWn8chgWYNyreiScMkmxg4U9u",
				"stake": "24146328727357015429360981746",
				"validator_stake_struct_version": "V1"
			}
		],
		"prev_block_hash": "9aHDvg6TV44qRSoaiYR98ZxaQNufs7vQXV6w6Jpy5oe9"
	},
	"id": "idontcare"
}`

const CLIENT_BLOCK_RESPONSE_NEXT_BLOCK = `
{
	"jsonrpc": "2.0",
	"result": {
		"approvals_after_next": [
			null,
			"ed25519:24pvVMA2ybxuk7fsCNAxDRnby5KQbGBM61T4Am74grDRuhiPbtYWBrubeSNWTejiAwiMZZt1zvLKSR8Djr4nDfHz",
			"ed25519:24pvVMA2ybxuk7fsCNAxDRnby5KQbGBM61T4Am74grDRuhiPbtYWBrubeSNWTejiAwiMZZt1zvLKSR8Djr4nDfHz",
			"ed25519:24pvVMA2ybxuk7fsCNAxDRnby5KQbGBM61T4Am74grDRuhiPbtYWBrubeSNWTejiAwiMZZt1zvLKSR8Djr4nDfHz",
			"ed25519:c78hanGiPzZ5iq9GPQET9pTh6J8pw5YgRGjtbNq35LuCzyTa5b4vdjzcAfHuRznfbTis77nF1aL6zm4CTJTesgU",
			"ed25519:65mYbzdjVUkWCh1wL81kZu96XphPP8X5McUVo2ScSKPgNiNBd3AsyR5XbJE7MGW5GnBwaqDPK8ft3yyRa3UMJnua",
			"ed25519:4akzNHFaa7w1LvaBTFKir9ExKStoRo44rm7YJ7XvtrigDnWmQ41EV7SyEFqcSqbDSznoxZybLQUV8ccCbia1daNT",
			"ed25519:4AGwZcCRk5WhCEuvEk12ANyJKHwwoLAPmGjU9Vqf7Xn7pDQcXw5sY8sPt3LazU7EYVaDUnZwWJUp2cAGHXkuLLyL",
			"ed25519:31anmHx3XEyPCnn7Mth5oppwbXoJbDQQjw3WjcDLGs8167RBE4WgCPaHn8kfHyhQ3tWWHudi1CFhy92yjJKPdLNK",
			"ed25519:4qLsTS1cF9ahcAnddUGjY7yFx2Sd8gJpp2dU3LjRpzT9vpG3grrGDcqCxVRAgjq7tEyuKXsbL7zyxVgpXjbicVwK",
			"ed25519:LtwgqDVPQWvomdx2zoXmfopgRgzLjxpovmjRXgetpZvc3E19iKjHbYtcs8FgGS4b6AT9GqgtoGfuLD1qdR94i2D",
			"ed25519:4pqiJapEEMS3czyAwM7QW4qecjT5u4EQkFa79rtCmSEACnKtxuU5PGBwsbZJkq7h8xhS24vN7d5AzszuKGWviNR9",
			"ed25519:38FxGgLRJMoD2cC3zX93c47iD51pGMvcVpPzCX9hSYfexJg8st7Ny4vr6U4sBfiyLeToqJTuoobEUzEts2eZXBxa",
			"ed25519:2Srg7nZ29C8ySxMKkMFXzFXj5i1RL4NGQ19GQryMmHcEYSVLFGBauhjydbEtaEQ5tzpZpMFPeGLnyV88GQSUHcPA",
			null,
			"ed25519:2LX76ZV8iB7ZyaVAAtpUjQDwKAshix3zLk9X5kF6gsn7oqMT6Rw6Mns3HZkD9M4mmMGEiUQqETw8P36Kymb1GUjb",
			"ed25519:2e39nRRS97kvfkjjGohtggubeTBGX8sqGSuL83nH1PYWDGoSANcUZqYeWZKxy8dzW44HEc9ptHYgGsynf6m4RY5S",
			"ed25519:2s6yBzV5D8VFS9hDsyqwHw3QQu4mQhjq4R6VHYMVXgbbogCe11eP4xZYUtw44gZrPawV3yxeqQH2RQFngdw5fABH",
			"ed25519:5kZ6EbdnhfwxdzwhEBarnMeNi5ng2UujvYNkafUVEeN39Bbap3WgiNz1j697WrW9Zw1HNNu4ZEGxk4ad96Z3e6rB",
			"ed25519:3qXYatuMPnxyRKyytzSBTtRxQ38Sm42Asf9jDo5MPoNnQVhiHBdiAydZWyrKfdqgHnibVc6Xxh6yPzSQxK67xCFu",
			null,
			"ed25519:5AnKpS5LaHayrW8pFoCkNugAEfSvniMJEzCSq1u4NPYTrkzsrLRiQ2SANVwA3PkXJrz6hd1abQCCWNhJMPsNNHQu",
			"ed25519:5D867Gg5xv9XiBWXMxhzx3cfY41moU5g7E62PyQLrEmvLSL5px67ojzasVd4whdqF3CzkN8wuuzGi2vvqqPNLPkr",
			"ed25519:5Bi9FH8gmnncJJNjpcNQx3AV12VpmF6Mk3pCGVvitBUrMMAKrYUEHh3knZpGJVWCVjP4TyxfKRwvCGVH2VNbHXMF",
			"ed25519:2J13vtY7vzxREcYQU4micZMpNskakdbvbxC5CMUnf4BSRf6my2nQ5g77GWSH4DNC9FTSW6ZQACJHfXyy9opMqLfT",
			"ed25519:sLUzhmxwGgVRLePVBvwMrW3Ny7E2ftWVnRAntbqF4sempASFMbwhjHvcBUfmNtJUSL9Qc5gEwbDgMMrriEuK2JK",
			"ed25519:2547xqoEPW9hR2Jh5FDgZDvuxmacMdPUdq4mqpJt18StSLoWN3B5ojSztBMdRaRNga5DWneNL9GViB712BJqYksh",
			"ed25519:3XKJohJC5Vr79FM5aVfjGkBP2Ck8hNEtEemmMRKfNh8NLQyVAED8rqyHhxSG2G7tnmt37tUgaZQcyNaQe9AC7zB3",
			"ed25519:wRkBDZg2MyGHZqhGYq8Pyv3uvu15jghNVWgdhmtsE5CFqzp4ws2YTCRnS4KUe3U7canCByh9hJHGetj9EHaGUop",
			null,
			"ed25519:5MiiKrUGXpcZB9VDVxUSpvKumFR9yiZgWEsVjHK4erJ4JEfzd7M17KnaaDWdLWn3w23drpqBLZCLxP6d1FqcvWP2",
			"ed25519:3WC83k8v1AqtK8QzUsNFWKSQrTNxbfxmtm19xDwnTDG5W22uzYb232eBAwALFqZjSbNifr1DXd25fyE7msM6kfjc",
			"ed25519:3UvniTBSgJPp8Mv1b8Z39pj7DSKZ3Epfy5xC7Mo4SAFDhgTZ7rxABtjT7tKj1S73JoREkzvdW2H1zRfeCRpWYyMU",
			"ed25519:36XryCFKF9tVv4x6FhaJT8iYfrZPCFYbjnmteNkzXQcJRiRp2MwivcYpvrkUUzFYMDuN4uSdYgozs3uPqgKha6Mb",
			null,
			null,
			"ed25519:5znLSC9mJRDEt5ozPc9cBisW5fn3matgmEcNBQtvvgpNNGgXYxHzW8aJkTmrovXRDiyWDbwkpY3GYbqPy66zGNSW",
			"ed25519:35uHJvJ8cmQGxHWsPjkBMg4SCmDEEYmLgUuqvQZBarqx5uck6apdi3SRp3AgSPDzT8tFuGCiXz8EHByHjDmoGbiz",
			"ed25519:45Qq5tSNJbzphyGerqEKCsEBq8bmrza5aferuEEijmhdgATdt6f4RDE9PDc86AwdURTLd7UVerkTtHheofa2YJet",
			"ed25519:3G1gba8V5YsFdKnQmwpGfcy47J6etLeBz57oZwdrRnqWboQd15TRzJxzfmrMgMn415CpwLFq3iXWBrUUA2B3ZiPe",
			null,
			null,
			"ed25519:5u63DbmzPiyB1R7DeCpKUAN1fJUTUpmh7FieGm6w1JWcJtHcD3EkMQGs5eoQ4XZbStACc3f9CSeQrz13hm7B2ipN",
			"ed25519:2vgaVE77b38bFYNJort92hRJQrtxZp13vLCK6WQqs9cbWAQfk5pMnPcUZis2z9rbk411QhmtWo9WPHHspAgMUEaf",
			"ed25519:TdUvco7vQAXqor6fBcwPBhyDaVKffYXknRB3T7cyWwDBNJ9etJtNje7wL6oQmUkQwndqwzKscNg8nKN38M5Fzdt",
			"ed25519:dYFyNK7uNQECEXzrj4eQZAGdkeKrVxnsR6u3rRDE43uJTBf1tXPffspeonwMuFx9DqsGg4DSRy6hPPmzdNQruNz",
			null,
			null,
			null,
			"ed25519:5vsSMabj5pz7um6fvVKwF6WyJvsaEZ8YjyeqgSxSkZGWB2Zm2yaV7QqzTnzurx4KT7Zhdvow4HjA3hBWt8Wt1ti1",
			null,
			null,
			"ed25519:3HfQt71AT6iVygpeNBHUaimx3iNnApbfTSsL5u65uyzkSHPCYwwSoQ7GfUCSuMp7HAm1cvpTf8RxzimKu9WeGa88",
			null,
			"ed25519:5ds369kTT4eUM1gcebAuuShPtft7LEZTA5oDwsxVv3Bazpb15WmDhSRuhUztGVTpDwMXijs68Gt7kUu3bD45KJpH",
			null,
			null,
			null,
			null,
			null,
			"ed25519:q7c7Mu5mKvZuBfpeRChGMbL1BZuwv72k2YvF9QoQHZE1yMRYnyQxvnAuHMiYLnnqKyD9PKA9ncssJfZcoL6jV2a",
			"ed25519:5u6PWvtS88g13Z5aS6y57uBfuDXvXTw7Hr9ZaxcicadfyHZCft9tq71heoUva9ewZLznWsCBy7JCJ7m75JG1CDPA",
			"ed25519:xjH8MR2JGH9ofpFPaU8GcoidsAohePQjtSi4M7T6SgXC1qZzhst6WLuguBaKTBqoPZU75N2Kkztfv3SKdyJKUQm",
			"ed25519:3nVu8XDkNep3UDWF8QVvf9NvfL37Z5DBFEXJ6VbsosRjpH8NSuU8DquYfd55rfDNHDUxY1yerk1grz1GvorhUon6",
			"ed25519:28qtBZAsbnBiZ6wPynzgTXuRm2fB4SPGmeBgyrfY61VSCNkr7LZ5zwLUxhrDUGQnLiaVS13tU9eBECvbocGZBgXE",
			null,
			"ed25519:M8ybFBsk3xZuXE48RwxSCVwyZB2srJVQ85cWazneyc1SQzuHXciKzouw3NXzwussKvpvvV4jsyPyEosfVmifnMm",
			"ed25519:38yS9p1AcoXiS7E4EMn9gpCvAppCrdvygvDQwnP5VTjHahTbyGLV67mre1k4x9TZ2JD36sffYZzh5BBgaXpNSCJF",
			"ed25519:G9H34TNeTP5QgifK9a6Y8PQVpnM5x7V2M7zSzsYCjdc1GUVsFFrMWPiJsigKnrV5pKi6yvWFUDwhYgXUPGGKZiY",
			"ed25519:4tC17LadtbHChDDvEJaGrsmc1Jj7F6PT7GQq9Ncd8tykG5tNYxfA9kXz57tvwRbzeZjqjPykAPY2KrN4XMs4M9sB"
		],
		"inner_lite": {
			"block_merkle_root": "3MBnipBo8GnqJisZN3uFjHLuvMusBSCjMaQmUmj5u4J6",
			"epoch_id": "GHmqgUX59irTdh31mtuEs3uEaPNBY5sQTZjEX5w7ASgW",
			"height": 86456070,
			"next_bp_hash": "9VPzyStHi4X2T7VAbfSTbLXEd8vjFP7wFJjYyjSJxQik",
			"next_epoch_id": "8nVTHDfxg2G8AWbKhVfFtnEb5jJeiXV2XBFsdyt2cif1",
			"outcome_root": "56KJ7kyW7aADwfDdNE4fz7pmPccBqkmxvnJ3nR1fewop",
			"prev_state_root": "2VJekkjBnP36c3sGo9P2YxkEu9dabK9r5VdMTt7jADLv",
			"timestamp": 1648810204507638699,
			"timestamp_nanosec": "1648810204507638699"
		},
		"inner_rest_hash": "GQHrWtXByznAWcawC7GoEMumZ3GUi2T82MXV56c2x8KS",
		"next_block_inner_hash": "8rHAfAgpXQKXTDWwEPvHwzKaBjG67nv4eNGSfc7A8FZ5",
		"next_bps": [
			{
				"account_id": "node1",
				"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
				"stake": "22949327592242450816363151898853",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "node0",
				"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
				"stake": "16944923507607057621836326590864",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "node2",
				"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
				"stake": "16894243398827941870356919783063",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "node3",
				"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
				"stake": "8577838094223400746241842212915",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "legends.pool.f863973.m0",
				"public_key": "ed25519:AhQ6sUifJYgjqarXSAzdDZU9ZixpUesP9JEH1Vr7NbaF",
				"stake": "5793326871499643941084500854531",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "staked.pool.f863973.m0",
				"public_key": "ed25519:D2afKYVaKQ1LGiWbMAZRfkKLgqimTR74wvtESvjx5Ft2",
				"stake": "4559762052294055739961541809028",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "masternode24.pool.f863973.m0",
				"public_key": "ed25519:9E3JvrQN6VGDGg1WJ3TjBsNyfmrU6kncBcDvvJLj6qHr",
				"stake": "3416574120678826701003147150326",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "01node.pool.f863973.m0",
				"public_key": "ed25519:3iNqnvBgxJPXCxu6hNdvJso1PEAc1miAD35KQMBCA3aL",
				"stake": "3061276782639300406837420592214",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "p2p.pool.f863973.m0",
				"public_key": "ed25519:4ie5979JdSR4f7MRAG58eghRxndVoKnAYAKa1PLoMYSS",
				"stake": "2958427611565856637171061933942",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "nodeasy.pool.f863973.m0",
				"public_key": "ed25519:25Dhg8NBvQhsVTuugav3t1To1X1zKiomDmnh8yN9hHMb",
				"stake": "1575068818350064235628643461649",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "tribe-pool.pool.f863973.m0",
				"public_key": "ed25519:CRS4HTSAeiP8FKD3c3ZrCL5pC92Mu1LQaWj22keThwFY",
				"stake": "1429199212043501677779067532132",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "chorusone.pool.f863973.m0",
				"public_key": "ed25519:3TkUuDpzrq75KtJhkuLfNNJBPHR5QEWpDxrter3znwto",
				"stake": "1278827676875609593894511486301",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "hotones.pool.f863973.m0",
				"public_key": "ed25519:2fc5xtbafKiLtxHskoPL2x7BpijxSZcwcAjzXceaxxWt",
				"stake": "1273529881837124230828073909315",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "foundryusa.pool.f863973.m0",
				"public_key": "ed25519:ABGnMW8c87ZKWxvZLLWgvrNe72HN7UoSf4cTBxCHbEE5",
				"stake": "1256081604638924285747937189845",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lunanova2.pool.f863973.m0",
				"public_key": "ed25519:9Jv6e9Kye4wM9EL1XJvXY8CYsLi1HLdRKnTzXBQY44w9",
				"stake": "1247431491303762172509349058430",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "chorus-one.pool.f863973.m0",
				"public_key": "ed25519:6LFwyEEsqhuDxorWfsKcPPs324zLWTaoqk4o6RDXN7Qc",
				"stake": "1110429050842727763339891353120",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "ni.pool.f863973.m0",
				"public_key": "ed25519:GfCfFkLk2twbAWdsS3tr7C2eaiHN3znSfbshS5e8NqBS",
				"stake": "1076903268858699791106964347506",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "cryptogarik.pool.f863973.m0",
				"public_key": "ed25519:FyFYc2MVwgitVf4NDLawxVoiwUZ1gYsxGesGPvaZcv6j",
				"stake": "840652974653901124214299092043",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "pathrocknetwork.pool.f863973.m0",
				"public_key": "ed25519:CGzLGZEMb84nRSRZ7Au1ETAoQyN7SQXQi55fYafXq736",
				"stake": "749739988926667488225409312930",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "stakely_v2.pool.f863973.m0",
				"public_key": "ed25519:7BanKZKGvFjK5Yy83gfJ71vPhqRwsDDyVHrV2FMJCUWr",
				"stake": "734779467803676488422251769143",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "solidstate.pool.f863973.m0",
				"public_key": "ed25519:DTDhqoMXDWhKedWpH7DPvR6dPDcXrk5pTHJw2bkFFvQy",
				"stake": "715205657993906057594050568659",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "aurora.pool.f863973.m0",
				"public_key": "ed25519:9c7mczZpNzJz98V1sDeGybfD4gMybP4JKHotH8RrrHTm",
				"stake": "703162032315675728652111978820",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "namdokmai.pool.f863973.m0",
				"public_key": "ed25519:9uGeeM7j1fimpG7vn6EMcBXMei8ttWCohiMf44SoTzaz",
				"stake": "699426128043696790256527911933",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "freshtest.pool.f863973.m0",
				"public_key": "ed25519:5cbAt8uzmRztXWXKUYivtLsT2kMC414oHYDapfSJcgwv",
				"stake": "697072950038835725218153979145",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "optimusvalidatornetwork.pool.f863973.m0",
				"public_key": "ed25519:BGoxGmpvN7HdUSREQXfjH6kw5G6ph7NBXVfBVfUSH85V",
				"stake": "661182931526239970852421432715",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "baziliknear.pool.f863973.m0",
				"public_key": "ed25519:9Rbzfkhkk6RSa1HoPnJXS4q2nn1DwYeB4HMfJBB4WQpU",
				"stake": "651150213650042597898598894903",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "blockscope.pool.f863973.m0",
				"public_key": "ed25519:6K6xRp88BCQX5pcyrfkXDU371awMAmdXQY4gsxgjKmZz",
				"stake": "649506414222131713576984442889",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "tagard.pool.f863973.m0",
				"public_key": "ed25519:3KyziFgx3PpzorJnMFifXU4KsK4nwPFaxCGWTHaFBADK",
				"stake": "646786097203475534304943885178",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "leadnode.pool.f863973.m0",
				"public_key": "ed25519:CdP6CBFETfWYzrEedmpeqkR6rsJNeT22oUFn2mEDGk5i",
				"stake": "644367778886663802105399198378",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "stakesstone.pool.f863973.m0",
				"public_key": "ed25519:3aAdsKUuzZbjW9hHnmLWFRKwXjmcxsnLNLfNL4gP1wJ8",
				"stake": "641198519157648602505664886163",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "basilisk-stake.pool.f863973.m0",
				"public_key": "ed25519:CFo8vxoEUZoxbs87mGtG8qWUvSBHB91Vc6qWsaEXQ5cY",
				"stake": "639918590440004706626411243128",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "shardlabs.pool.f863973.m0",
				"public_key": "ed25519:DxmhGQZ6oqdxw7qGBvzLuBzE6XQjEh67hk5tt66vhLqL",
				"stake": "637803882455578964186296090355",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "al3c5.pool.f863973.m0",
				"public_key": "ed25519:BoYixTjyBePQ1VYP3s29rZfjtz1FLQ9og4FWZB5UgWCZ",
				"stake": "636854880374440657378246667596",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "dehashed.pool.f863973.m0",
				"public_key": "ed25519:EmPyD1DV9ajWJxjNN8GGACMyhM9w14brwNwYA5WvVaw",
				"stake": "635224150718459403099965806552",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "machfund.pool.f863973.m0",
				"public_key": "ed25519:G6fJ79oM6taQGhHeQZrg8N36nkCPMEVPyQMHfFT2wAKc",
				"stake": "634686788251976758263963874506",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "blockngine.pool.f863973.m0",
				"public_key": "ed25519:CZrTtCP6XkkxWtr3ATnXE8FL6bcG5cHcxfmdRgN7Lm7m",
				"stake": "633656065475669726280826427959",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "grassets.pool.f863973.m0",
				"public_key": "ed25519:3S4967Dt1VeeKrwBdTTR5tFEUFSwh17hEFLATRmtUNYV",
				"stake": "622722987982775798532829252304",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "bflame.pool.f863973.m0",
				"public_key": "ed25519:4uYM5RXgR9D6VAGKHgQTVNLEmCgMVX7PzpBstT92Me6R",
				"stake": "617234461115345372278772960093",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "shurik.pool.f863973.m0",
				"public_key": "ed25519:9zEn7DVpvQDxWdj5jSgrqJzqsLo8T9Wv37t83NXBiWi6",
				"stake": "616327809807619407716759066614",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "dsrvlabs.pool.f863973.m0",
				"public_key": "ed25519:61ei2efmmLkeDR1CG6JDEC2U3oZCUuC2K1X16Vmxrud9",
				"stake": "613792106557214713239288385761",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "zetsi.pool.f863973.m0",
				"public_key": "ed25519:6rYx5w1Z2pw46NBHv6Wo4JEUMNtqnDGqPaHT4wm15YRw",
				"stake": "611882168159257611258042281605",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "n0ok.pool.f863973.m0",
				"public_key": "ed25519:D6Gq2RpUoDUojmE2vLpqQzuZwYmFPW6rMcXPrwRYhqN8",
				"stake": "594349395199079126466241101938",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "chelovek_iz_naroda.pool.f863973.m0",
				"public_key": "ed25519:89aWsXXytjAZxyefXuGN73efnM9ugKTjPEGV4hDco8AZ",
				"stake": "592739793772796190513231168872",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lavenderfive.pool.f863973.m0",
				"public_key": "ed25519:AzwAiLDqprZKpDjhsH7dfyvFdfSasmPTjuJUAHfX1Pg4",
				"stake": "586231008421809079867645695624",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "latenthero.pool.f863973.m0",
				"public_key": "ed25519:EQqmjRNouRKhwGL7Hnp3vcbDywg2Boj6to2gmnXybhEM",
				"stake": "579738101137715103577294987834",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "tayang.pool.f863973.m0",
				"public_key": "ed25519:G9XWX55MfWEpT84ckcsJxVTKeZK4WqBGJX3xVpnPB5vv",
				"stake": "563498889920635651950224126233",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "smcvalidator.pool.f863973.m0",
				"public_key": "ed25519:pG4LYsyoAa8yWYG9nsTQ5yBcwke51i3VqeRcMVbE9Q7",
				"stake": "555422197586970576403131175346",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "everstake.pool.f863973.m0",
				"public_key": "ed25519:4LDN8tZUTRRc4siGmYCPA67tRyxStACDchdGDZYKdFsw",
				"stake": "546400197607367519956748211889",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "rossi-validator.pool.f863973.m0",
				"public_key": "ed25519:2eRx2c3KX9wFd3EzuuajFQoSxRTKDqSbxcF13LfkrxCR",
				"stake": "545396693549586230215202952473",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "infiniteloop.pool.f863973.m0",
				"public_key": "ed25519:2fbiLqksH5viWXYoteyfKP9qQawkRKw4YogRFcvG3Z7f",
				"stake": "538321976932135835213436874121",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lusienda.pool.f863973.m0",
				"public_key": "ed25519:HdQb2HEiaMgvUdemTt5rkrFbxTpzZyELvg1Vov4LQAGU",
				"stake": "509015164869674763004419847436",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "ino.pool.f863973.m0",
				"public_key": "ed25519:B75h2eqpaMgh6WkAvgnz2FsEC9s5TwVx7zwTjqXKfRs6",
				"stake": "494974817444468749939621071716",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "pontiff.pool.f863973.m0",
				"public_key": "ed25519:4i8j7nwNyy18hfARtrVpckT8MiicdCXuWBX1TubdMb5Y",
				"stake": "478587210879643963063840990682",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "kiln.pool.f863973.m0",
				"public_key": "ed25519:Bq8fe1eUgDRexX2CYDMhMMQBiN13j8vTAVFyTNhEfh1W",
				"stake": "96608509421037438882028377566",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "nodemeister.pool.f863973.m0",
				"public_key": "ed25519:85EMyaNGMFuHK2RDH7KHno6fVYBR6iykUXHPPmFTGuTB",
				"stake": "47021543808070096585479049932",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "nala.pool.f863973.m0",
				"public_key": "ed25519:Fzwndob2h5PFdEuwo9eRFJV3BLLurcNaw2SGob5rMPEn",
				"stake": "44766587364445748049092546945",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "happystake.pool.f863973.m0",
				"public_key": "ed25519:3APqZiwzeZLzgfkJyGGTfepDYHA2d8NF1wZi4mCpZnaJ",
				"stake": "43959988855512773720415910025",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "ibb.pool.f863973.m0",
				"public_key": "ed25519:7gvdHhcMcXT1jMZoxDKy7yXnRiPVX1tAFTa7HWTHbe8C",
				"stake": "42001690004861681144621857517",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "mateennala.pool.f863973.m0",
				"public_key": "ed25519:9kNpQKUKzhc1AiFSEoZcTNapTnywjbXBPngH3EDpD1tw",
				"stake": "40056014128143748170300000000",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "wolfedge-capital-testnet.pool.f863973.m0",
				"public_key": "ed25519:CQEMcPQz6sqhAgoBm9ka9UeVcXj5NpNpRtDYYGkPggvg",
				"stake": "37464905110868615156797728096",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "jstaking.pool.f863973.m0",
				"public_key": "ed25519:fui1E5XwnAWGYDBSQ3168aDfsW1KDFH8A7nBHvZiqGv",
				"stake": "36368375383183646876651257216",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "dariya.pool.f863973.m0",
				"public_key": "ed25519:A5Rx38TsNKWXzF5o18HpaRrPeBzv3riqur51bqhU1Qbp",
				"stake": "36211347514033914937590010268",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "4ire-pool.pool.f863973.m0",
				"public_key": "ed25519:EWPSvYN9pGPMmCLjVxx96stWdqksXNSGnfnuWYn9iiE5",
				"stake": "33869896086305183386478534323",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lionstake.pool.f863973.m0",
				"public_key": "ed25519:Fy6quR4nBhrEnDyEuPWoAdBP5tzNbuEZsEd91Q5pQnXB",
				"stake": "33765876364623459491244697143",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "zentriav2.factory.colorpalette.testnet",
				"public_key": "ed25519:4rCwSFzJ2e6suD5Yi7pgLidcAJ8Zt9BXieLzVedJDwmE",
				"stake": "30596434283244809799848018489",
				"validator_stake_struct_version": "V1"
			},
			{
				"account_id": "lastnode.pool.f863973.m0",
				"public_key": "ed25519:811gesxXYdYeThry96ZiWn8chgWYNyreiScMkmxg4U9u",
				"stake": "24146328727357015429360981746",
				"validator_stake_struct_version": "V1"
			}
		],
		"prev_block_hash": "4E2VN7cUVSb8ek761H4cRo57ERTWBKbcB9uEBDS2cWhD"
	},
	"id": "idontcare"
}`

func TestValidateAndUpdateHeadValidBlockNextEpoch(t *testing.T) {
	lightClientBLockViewPreviousEpoch, err := getLightClientBlockView(t, CLIENT_RESPONSE_PREVIOUS_EPOCH)
	require.Nil(t, err)

	lightClientBLockViewCurrentEpoch, err := getLightClientBlockView(t, CLIENT_BLOCK_RESPONSE_NEXT_BLOCK)
	require.Nil(t, err)

	liteClient := NewLiteClientFromCheckpoint(*lightClientBLockViewPreviousEpoch)
	result, err := liteClient.ValidateAndUpdateHead(lightClientBLockViewCurrentEpoch)
	require.Nil(t, err)

	assert.True(t, result)
}

func TestValidateAndUpdateHeadValidBlockPreviousEpoch(t *testing.T) {
	lightClientBLockViewPreviousEpoch, err := getLightClientBlockView(t, CLIENT_RESPONSE_PREVIOUS_EPOCH)
	require.Nil(t, err)

	lightClientBLockViewCurrentEpoch, err := getLightClientBlockView(t, CLIENT_BLOCK_RESPONSE_NEXT_BLOCK)
	require.Nil(t, err)

	liteClient := NewLiteClientFromCheckpoint(*lightClientBLockViewCurrentEpoch)
	result, err := liteClient.ValidateAndUpdateHead(lightClientBLockViewPreviousEpoch)
	require.Nil(t, err)

	assert.False(t, result)
}

const CLIENT_PROOF_RESPONSE = `{
	"jsonrpc": "2.0",
	"result": {
		"block_header_lite": {
			"inner_lite": {
				"block_merkle_root": "D5nnsEuJ2WA4Fua4QJWXa3LF2TGoAqhrW8fctFh7MW2s",
				"epoch_id": "7e3Vkbngf36bphkBVX98LoRxpoqhvZJbL5Rgb3Yfccy8",
				"height": 86697768,
				"next_bp_hash": "Hib973UH8xTq4ReP2urd1bLEaHGmjwWeHCyfQV4ZbHAv",
				"next_epoch_id": "7AEtEQErauvaagnmmDsxw9qnYqBVuTKjSW4P7DVwZ5z3",
				"outcome_root": "AZYywqmo6vXvhPdVyuotmoEDgNb2tQzh2A1kV5f4Mxmq",
				"prev_state_root": "6BWNcpk4chiEXWRWbWum5D4zutZ9pomfwwbmjanLp4sv",
				"timestamp": 1649062589965425850,
				"timestamp_nanosec": "1649062589965425850"
			},
			"inner_rest_hash": "DeSCLALKLSEX6pjKVoStCUq3ixkzK4v958TMkdPp1fJJ",
			"prev_block_hash": "Ae7sLAjvHs3gkiU2vFt8Vdxs5RmVUwyxyCwbnqnTkckQ"
		},
		"block_proof": [
			{
				"direction": "Right",
				"hash": "BNmeYcDcNoVXgXZyzcoyJiN5UiyLeZTvwSHYRpSfw9fF"
			},
			{
				"direction": "Right",
				"hash": "A7HaT2EGxrhJhDK2muP56b6j6c5JL1VAFPE45iB4cxsf"
			},
			{
				"direction": "Left",
				"hash": "AjhQk267UxRgxrTtLyjHrVoid7DPRN67aki8GJZttnu4"
			},
			{
				"direction": "Left",
				"hash": "4qyS6XAo8fNLYeGQJVN31D8ncr4TfmrvSe3cursw8oM7"
			},
			{
				"direction": "Right",
				"hash": "28y98e3vha3vHmkBhgREgxjLzjP7JzfVeu6H6yDHMh4V"
			},
			{
				"direction": "Left",
				"hash": "CJRqXDJy8L1oEGJDPxXgPuQhrFmLosoFQAf79Dyfrw3z"
			},
			{
				"direction": "Left",
				"hash": "CGaUbgtx9UFf7sZAe5fLdy1ggb5ZGg2oC3LmT2SgnCbz"
			},
			{
				"direction": "Left",
				"hash": "EjFednH4uWzcYNJzrfiBPbcDEvVTi7u7MEDFbcJfdPYf"
			},
			{
				"direction": "Right",
				"hash": "HAxQFR7SS2gkNUZ4nfSNefo3N1mxsmn3n7sMzhBxxLi"
			},
			{
				"direction": "Left",
				"hash": "KQa9Nzw7vPnciog75ZGNriVU7r4aAqKErE15mEBd3sS"
			},
			{
				"direction": "Left",
				"hash": "ByNUgeXrsQpeCNeNEqpe8ASw2bh2BfY7knpLaQe1NtXv"
			},
			{
				"direction": "Left",
				"hash": "ByrTiguozXfUaufYN8MuWAx7jL1dhZJ7bLzJjpCQjvND"
			},
			{
				"direction": "Left",
				"hash": "DvV6ak7n9wP1TQ1a97P81b81xJq1EdnERp8r3GFdP7wU"
			},
			{
				"direction": "Left",
				"hash": "Gga62BEfbomV8ZNz3DkPQEFf6UbEqMKngwNAp5zDDoki"
			},
			{
				"direction": "Left",
				"hash": "76U6DMh4J4VB5sfVVNRpSTeB4SEVt4HPqhtQi2izGZxt"
			}
		],
		"outcome_proof": {
			"block_hash": "5aZZNiqUVbXXvRjjf1FB8sbXG3gpJeVCw1bYeREXzHk2",
			"id": "8HoqDvJGYrSjaejXpv2PsK8c5NUvqhU3EcUFkgq18jx9",
			"outcome": {
				"executor_id": "relay.aurora",
				"gas_burnt": 2428395018008,
				"logs": [],
				"metadata": {
					"gas_profile": null,
					"version": 1
				},
				"receipt_ids": [
					"8hxkU4avDWFDCsZckig7oN2ypnYvLyb1qmZ3SA1t8iZK"
				],
				"status": {
					"SuccessReceiptID": "8hxkU4avDWFDCsZckig7oN2ypnYvLyb1qmZ3SA1t8iZK"
				},
				"tokens_burnt": "242839501800800000000"
			},
			"proof": [
				{
					"direction": "Right",
					"hash": "B1Kx1mFhCpjkhon9iYJ5BMdmBT8drgesumGZoohWhAkL"
				},
				{
					"direction": "Right",
					"hash": "3tTqGEkN2QHr1HQdctpdCoJ6eJeL6sSBw4m5aabgGWBT"
				},
				{
					"direction": "Right",
					"hash": "FR6wWrpjkV31NHr6BvRjJmxmL4Y5qqmrLRHT42sidMv5"
				}
			]
		},
		"outcome_root_proof": [
			{
				"direction": "Left",
				"hash": "3hbd1r5BK33WsN6Qit7qJCjFeVZfDFBZL3TnJt2S2T4T"
			},
			{
				"direction": "Left",
				"hash": "4A9zZ1umpi36rXiuaKYJZgAjhUH9WoTrnSBXtA3wMdV2"
			}
		]
	},
	"id": "idontcare"
}`

func getRPCLightClientExecutionProofResponse(payload []byte) *types.RPCLightClientExecutionProofResponse {
	type result struct {
		Result types.RPCLightClientExecutionProofResponseJSON `json:"result"`
	}
	var r result
	err := json.Unmarshal(payload, &r)
	if err != nil {
		log.Fatal(err)
	}

	result2, err := r.Result.IntoRPCLightClientExecutionProofResponse()
	if err != nil {
		log.Fatal(err)
	}

	return result2
}

func TestSerializeRPCLightClientExecutionProofResponse(t *testing.T) {
	response := getRPCLightClientExecutionProofResponse([]byte(CLIENT_PROOF_RESPONSE))
	assert.Equal(t, base58.Encode(response.OutcomeProof.Outcome.ReceiptIds[0][:]), "8hxkU4avDWFDCsZckig7oN2ypnYvLyb1qmZ3SA1t8iZK")
}

func TestValidateTransaction(t *testing.T) {
	parsedResponse := getRPCLightClientExecutionProofResponse([]byte(CLIENT_PROOF_RESPONSE))
	assert.Equal(t, base58.Encode(parsedResponse.BlockHeaderLite.InnerLite.BlockMerkleRoot[:]), "D5nnsEuJ2WA4Fua4QJWXa3LF2TGoAqhrW8fctFh7MW2s")
	executionOutcomeHash, err := calculateExecutionOutcomeHash(&parsedResponse.OutcomeProof.Outcome, parsedResponse.OutcomeProof.ID)
	require.Nil(t, err)

	shardOutcomeRoot, err := computeRootFromPath(parsedResponse.OutcomeProof.Proof, *executionOutcomeHash)
	require.Nil(t, err)

	blockOutcomeRoot, err := computeRootFromPath(parsedResponse.OutcomeRootProof, sha256.Sum256(shardOutcomeRoot[:]))
	require.Nil(t, err)

	assert.Equal(t, "AZYywqmo6vXvhPdVyuotmoEDgNb2tQzh2A1kV5f4Mxmq", base58.Encode(blockOutcomeRoot[:]))

}
