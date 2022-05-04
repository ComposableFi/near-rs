package blockchain_connector

import (
	"encoding/json"
	"log"
	"testing"

	"github.com/ComposableFi/near-trustless-bridge/near-lite-relayer-go/types"
	"github.com/stretchr/testify/assert"
)

func TestNearNetwork(t *testing.T) {
	testnet := Testnet
	mainnet := Mainnet
	assert.Equal(t, "testnet", testnet.ToString())
	assert.Equal(t, "mainnet", mainnet.ToString())
	assert.Equal(t, "https://rpc.testnet.near.org", testnet.getBaseUrl())
	assert.Equal(t, "https://rpc.mainnet.near.org", mainnet.getBaseUrl())
}

func TestUnmarshalLightClientBlockView(t *testing.T) {
	payload := `{
		"jsonrpc": "2.0",
		"result": {
			"approvals_after_next": [
				"ed25519:4qnb1YmQngt9X3M88igWTWWPxX8GLwjYh6nHYYBGhZs5vFP5JxRNS8MqTNjn9eBebkd5mw72cM5emDKVfMY7hMrc",
				null,
				"ed25519:4qnb1YmQngt9X3M88igWTWWPxX8GLwjYh6nHYYBGhZs5vFP5JxRNS8MqTNjn9eBebkd5mw72cM5emDKVfMY7hMrc",
				"ed25519:4qnb1YmQngt9X3M88igWTWWPxX8GLwjYh6nHYYBGhZs5vFP5JxRNS8MqTNjn9eBebkd5mw72cM5emDKVfMY7hMrc",
				"ed25519:5MyB1oXRDgbiHZYh9Dx1f8PdVRrjRoDHSzzLHBP2aFcRsXXnJH52bFDBXWiWkgVqUxsuVL7MYZBsQBu6DbxKGgV",
				null,
				"ed25519:cUup31CKawSwX9pzQ6gGQrvdgTkeRJWh1fZ8EwM8ovKXjCVV6TmumyEcS7Att2sjzN3exQnsVULcKs4EiHJfv91",
				"ed25519:4xRxbVTTqgHCAxRj1VwcY5k1STLJAr3Mc96oCHAKFVqFJ1voA9QDKnemBQs2GNADcmT3bVaLAyjsGCyB5fkqAu5B",
				"ed25519:4ov9hLpCT7HnvMmj9rq7uYB3yhsbED7JHtkSjL4VvwqjtJpexLSCqjqMrhkUUqZQP8CDvMb2Ctkerf3Mq4C5mkre",
				"ed25519:sPHfZQCXBKvuipeveoXDFUZVnwoguF2Tc4g3uvGP9dJAnnTYsVwo6xPnUFrzRSgTFLDmtwsveh6766oHTD3Af6B",
				"ed25519:5xgL6FQ2CaH3sfUPPhh76My8nq1GrBrw3Lpqjphex1mq9xBKSWmHFzxWwgx6C4wK29KWcgvL4Ja9Qpe5VdTeQhn9",
				"ed25519:5Jo2eBAAkn4oRsTPP67SL42mtp1475KVeDchgiAjVmjPnEKJuJngyTKePfdvwadzuPAiNyhUbdKaZLXaazci6sP9",
				null,
				"ed25519:25bS3gtK65TZ7zaLvrWJ9oX6PzjkAwjrpp8VxfucMCXiVCbRZV4g2J29oRhRG89j2WQ1wyFgyWxESjJez2UvGy4z",
				null,
				"ed25519:4JyzLm5hBdF7ia94zvwr5r79coYoDTpU3WghUy4ZsYdN3yQF8BErY6rvnu24Em9mNBxM9n4GwdZfjYnvqtSgULiJ",
				"ed25519:2kRp8kpf5cr7v6t1y8evKbHFTMqY7e7Pern98iekU4Yk5NaryjjMJ3BKfhF3njeFAnDa3uNL2cDBH1ijt8XmMAgB",
				"ed25519:2pGfFsjpYRSay4BHKip2gj2Hbw2NxuYmYynhHnXNWaYfSrBFneKNsJ4Y1mEXUs2LFD7gDVoJDxnq9ruzwAhacRSt",
				"ed25519:4oJMjCLhg3sUfebSaXkBUzURTcTZAM3xrrPyLjpGT9oBR4MJudrutHutWUBcDDBinBHn8uCS6M1PpRKdSvWAYrbZ",
				null,
				"ed25519:3uSNUn19akyCWB6TdsBEmeiqLWPsUpC8Zy9QG8ha2HfBF31d2iftqW39h1GbBjGmKxaLGDQC8bbsiawKmpGSKdGB",
				"ed25519:2Z1tKT5ijjMYMDPJob2YYMLeyjQPK4ba2nLWypHqcqv5fGQHMLnXh8ex8vuAgmvTYQcS7bx2xwWG1uaTYy3j5jAR",
				null,
				"ed25519:3afmQkjj64CbYtRtuedKVpK3hhpk1kmcPvFYL4QVPKfbrh53UPjMzqSHbsRp35jvvYVj83bqZH7rbAjcHDXGyPoE",
				"ed25519:5rd4N5TFkURkYeDUMPgABGpaeK5314Ws1id9XG2FmfRH2C6ARTBLxes2EwWSEJeLeeNZLUGBBrPV6wnKLUuBJMPy",
				"ed25519:4dH1AKv26Gx7fvnZQrGcnmzz4GoJML6QzCN26PEssfwJgTn7rA9MbpXSwEMstdwqrZjCgp1tDFmVoKRqJNERahq2",
				"ed25519:ZnoNnE9UHdpogTCP4tiw4LQoEkLwZ6wUpZNJNJm9YBa8CTjuTECJrRc6wmu53WRS5GRq7hFc5EkxYSRZXof1SDq",
				"ed25519:QUYri7ZY1dD6mbMQ6yMV8HEaQemucGBoztDKzbsFmBXvxYmvuGTLiMnDBDL8cTyqA66kpNsKKEhU6iGXCW4P13L",
				"ed25519:3P5jVWfqyGLKYps1ZE5ns79GsnHc6wSMWtcsroUMa4z2mpoyoDYSgGqRmr2mrkNEtMgivZWmSDpmsCAZB5tMyoti",
				"ed25519:2rDPtSYHC7APKxSSR3C5JwdnT275RU4JfQBqzNdPTh7DNDbnYtscA1Rg4iZNZFmcnTtbrPkQmyK9fh816XFu8kuX",
				null,
				"ed25519:5rpHvAHhqXWMzZDC8WPHDQZdai7QEeuDWVXHhyvNvfp2nFtSULKyToGkMTaXix5oCw7acxZgBvJfcacvoY2Dzq22",
				"ed25519:3cKj9PpUsqi7VdcbDdabcmnRLsMiNxYzyTd47JcL96iSE2iT7Pe9sZG3usrHibm3XTugGoZyA6BSWpmzRGGJsoW5",
				"ed25519:3QuYZYKXYX89uNbbri9RzRGZM67ZfLLUo1J2g296xKwN4zX2VLe7Bo24HofrKVBfcmfcGWy1nBFfQYS57dCaAEyf",
				null,
				"ed25519:47FjJp4wPvTuDwDedXhgGP6NSfYsJSYEf5jfHpodVdHw4fGxjLidwQ6aLmo6uSvyq4UU9AopLGQJGxEUvpX3U9a",
				"ed25519:3mCD1VXi6A4fFL7pRoMP7wSPJMmXq4PM8X3xbCRNgtpCGVPCRLi3581hMiJfnsKwxWeyV93ffLnGXFH5CJr6ZswH",
				"ed25519:3hy8kJigNEsHB3hfods7RgjRSDrKaHbMWJb8GNC6m7t94PApevVFErCiUC476nTMPkmjgM3iRhMSjwDcz3eb2e7E",
				null,
				"ed25519:4ouph4FajqeCq9oXCXP9eUgtSz3oLbCvDULKtynw4Cm59nhkRJZ65ABQH7PhRe3DGSEw1XPS83fV6JqnzsRWSznZ",
				null,
				"ed25519:3RZnZvgUneVHQd3Pub4cJxXaogAKEu1G8RSxAjc3ki3pWUasRqEzh9AF9f9zXHzqC6U4mvoi9DfScJPePBC8uSxc",
				"ed25519:4Q7k6MEqFsikhHRifVKFcCppmGnBYJJzNdBZhLYYb925jr11jd2XLej26ZHnfSnBWR2d143vWhCkEvzQVsrH3bfu",
				"ed25519:2oLdWVSXPMdESauS3Zoyb4dwhPgMxh5RbZ9q1fJuELGsNdTkTe7SKXTsreBHTwQZQrwLYHhT4yCdzp99aP73QYF9",
				null,
				"ed25519:2RJvwxT8HJZpNA9myPQob4BKpZ1HBekCVLAXN9Ej884qjdLsJakqmn7FLo2QwzshW7KpMPMD3t9tD1xz1YnGKysm",
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				"ed25519:3tpbXQandir2WXNJPrEqDoQJDWbPNSENNWQHNqvWSUWSzNEMnhRb36iAoZnkqtGVc3UbNkpWsUrq9VddStMoBq1N",
				null,
				null,
				null,
				null,
				null,
				null,
				"ed25519:6NCJnvs8T3L19hgS6pzwE3MrPNhz8MY6Ab6kcWEqqZYBABdKPdcvPziDs8cG4Fb73iGA1Psn1ekMqYuusBJgjYz",
				"ed25519:5XUB69RjrdAH5WzEyaaQdZU5PQqp8ibUpaVBcDWnH8yCzMivQdQhtie1g5xzVyERGJ1J5oQ5C7DsCUF4GV3Sh2BF",
				null,
				"ed25519:3PGsf24eCcEKzij5DKsjCL9mgxKuUX8MgwK8pjbyp4fSasMpiRoYvDsCfsU3QUUd1wL1e68Gd8e3hGhMFS3WVPhi",
				null,
				"ed25519:2mSEaRQh9imTsjh2sh76rBPsB4Ve7ue3oCCiCHEzJdnzKrLYARuaVLMxSicSa956gEnDnNSYg6g8nVkwqXcZBUw9",
				null,
				"ed25519:Li2QgyHnriUs1bUAHsZQd2Js6Arj1hrmCdzTchPWvtUEqGuxW5Syfu4HUpqxsT9kC1MYYtiKMkuQuAFjG2UGz8B",
				"ed25519:45T7qD4RvqF36MyCLP8Eh5H7DLPUGK6qn7ZXYjFFGLR6wm9YxX1cxiVnGwAdN4HVVq3jneYUZMfD82mjhfLGHL3Y",
				"ed25519:Kf88iQZnx8mAX6npFSym4Lfg7tpjJ2tHBVdnkJzBRQi8MjLgAqZpqzxPaTv2kK4aDj9uz4xj87nMbm6NJTn76ek",
				"ed25519:4E4yPzGTJ47UqJXRS59F9Ymcc7GgTJcAeawP2XSAgcnsE4V3Hh8weEfoXozDFeq6DXmy3Fb47uRqzytFDhjZeUFB",
				null,
				"ed25519:2W67raEfrmSZh2Wut57Na5k9oP4NgbRNUmsZds95STkJm2CsLREAU3voqQMjr2i5uBv5z5mbcuz9k3ncim4t5ACT",
				null,
				null,
				null,
				null,
				null
			],
			"inner_lite": {
				"block_merkle_root": "3aTamyUKPYRxKYP3oHaNhJ7LiaKikuBmU1MA5VK6zk5S",
				"epoch_id": "Ad5SqwwdfwdynaXtuzoSzMgjm6m9oZPbxeQf442hP2G2",
				"height": 87002986,
				"next_bp_hash": "2VQp3KoxaSA71PQ13BgBaM7ZuykjKtraSECWqv9GBv3y",
				"next_epoch_id": "2anw7wLLXcUfj3SjK4BBz6K6TNBFM9M1qY9PwmNeo6TU",
				"outcome_root": "CrgUswudNyDeUttTspjxE7RWE88bteAjf9Zz23DvizUa",
				"prev_state_root": "Fk58zL7meg9LrFB8XRheH3rYJxAKsKFxUzXxe9aShJNp",
				"timestamp": 1649382016925706604,
				"timestamp_nanosec": "1649382016925706604"
			},
			"inner_rest_hash": "7LXscagbNGWxPZ8sT85o9PzqtqfHRvacF2ENuAa1SjjC",
			"next_block_inner_hash": "9fMiayddBTK9ur97YFr6VQvsrxL6JrXGbXZwEof8a8RQ",
			"next_bps": [
				{
					"account_id": "node1",
					"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
					"stake": "23274801326131528968305033242063",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "node0",
					"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
					"stake": "17185241116143438438770586430105",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "node2",
					"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
					"stake": "17133842247994491770559252551878",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "node3",
					"public_key": "ed25519:ydgzeXHJ5Xyt7M1gXLxqLBW1Ejx6scNV5Nx2pxFM8su",
					"stake": "8699491375000419926807360746817",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "legends.pool.f863973.m0",
					"public_key": "ed25519:AhQ6sUifJYgjqarXSAzdDZU9ZixpUesP9JEH1Vr7NbaF",
					"stake": "5872411514783945695604674147839",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "staked.pool.f863973.m0",
					"public_key": "ed25519:D2afKYVaKQ1LGiWbMAZRfkKLgqimTR74wvtESvjx5Ft2",
					"stake": "4616761353256487760503635942846",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "masternode24.pool.f863973.m0",
					"public_key": "ed25519:9E3JvrQN6VGDGg1WJ3TjBsNyfmrU6kncBcDvvJLj6qHr",
					"stake": "3465311974110476958992383998314",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "01node.pool.f863973.m0",
					"public_key": "ed25519:3iNqnvBgxJPXCxu6hNdvJso1PEAc1miAD35KQMBCA3aL",
					"stake": "3104655577494829040348599989833",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "p2p.pool.f863973.m0",
					"public_key": "ed25519:4ie5979JdSR4f7MRAG58eghRxndVoKnAYAKa1PLoMYSS",
					"stake": "3000282040551821064774048455993",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "nodeasy.pool.f863973.m0",
					"public_key": "ed25519:25Dhg8NBvQhsVTuugav3t1To1X1zKiomDmnh8yN9hHMb",
					"stake": "1597496464725551659493577732834",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "sweden.pool.f863973.m0",
					"public_key": "ed25519:2RVUnsMEZhGCj1A3vLZBGjj3i9SQ2L46Z1Z41aEgBzXg",
					"stake": "1593298504905326828358520977021",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "tribe-pool.pool.f863973.m0",
					"public_key": "ed25519:CRS4HTSAeiP8FKD3c3ZrCL5pC92Mu1LQaWj22keThwFY",
					"stake": "1449468513072685151768869843652",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "chorusone.pool.f863973.m0",
					"public_key": "ed25519:3TkUuDpzrq75KtJhkuLfNNJBPHR5QEWpDxrter3znwto",
					"stake": "1297002676281628201359396765773",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "foundryusa.pool.f863973.m0",
					"public_key": "ed25519:ABGnMW8c87ZKWxvZLLWgvrNe72HN7UoSf4cTBxCHbEE5",
					"stake": "1273634249615861636098597816587",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "lunanova2.pool.f863973.m0",
					"public_key": "ed25519:9Jv6e9Kye4wM9EL1XJvXY8CYsLi1HLdRKnTzXBQY44w9",
					"stake": "1265231114849705735813839875221",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "chorus-one.pool.f863973.m0",
					"public_key": "ed25519:6LFwyEEsqhuDxorWfsKcPPs324zLWTaoqk4o6RDXN7Qc",
					"stake": "1126015500431708369339132799515",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "ni.pool.f863973.m0",
					"public_key": "ed25519:GfCfFkLk2twbAWdsS3tr7C2eaiHN3znSfbshS5e8NqBS",
					"stake": "1092176238139452434975295956501",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "cryptogarik.pool.f863973.m0",
					"public_key": "ed25519:FyFYc2MVwgitVf4NDLawxVoiwUZ1gYsxGesGPvaZcv6j",
					"stake": "852585447821286427901174628724",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "pathrocknetwork.pool.f863973.m0",
					"public_key": "ed25519:CGzLGZEMb84nRSRZ7Au1ETAoQyN7SQXQi55fYafXq736",
					"stake": "760367584547832852556626263037",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "stakely_v2.pool.f863973.m0",
					"public_key": "ed25519:7BanKZKGvFjK5Yy83gfJ71vPhqRwsDDyVHrV2FMJCUWr",
					"stake": "745519253047130108738185834017",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "solidstate.pool.f863973.m0",
					"public_key": "ed25519:DTDhqoMXDWhKedWpH7DPvR6dPDcXrk5pTHJw2bkFFvQy",
					"stake": "725217632123986614414259621584",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "aurora.pool.f863973.m0",
					"public_key": "ed25519:9c7mczZpNzJz98V1sDeGybfD4gMybP4JKHotH8RrrHTm",
					"stake": "713069060387458943144436503320",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "namdokmai.pool.f863973.m0",
					"public_key": "ed25519:9uGeeM7j1fimpG7vn6EMcBXMei8ttWCohiMf44SoTzaz",
					"stake": "709095141861030523843835425150",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "freshtest.pool.f863973.m0",
					"public_key": "ed25519:5cbAt8uzmRztXWXKUYivtLsT2kMC414oHYDapfSJcgwv",
					"stake": "706959037615658364804869191682",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "optimusvalidatornetwork.pool.f863973.m0",
					"public_key": "ed25519:BGoxGmpvN7HdUSREQXfjH6kw5G6ph7NBXVfBVfUSH85V",
					"stake": "670290764969714430825093626839",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "baziliknear.pool.f863973.m0",
					"public_key": "ed25519:9Rbzfkhkk6RSa1HoPnJXS4q2nn1DwYeB4HMfJBB4WQpU",
					"stake": "660435358615020450442916363923",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "blockscope.pool.f863973.m0",
					"public_key": "ed25519:6K6xRp88BCQX5pcyrfkXDU371awMAmdXQY4gsxgjKmZz",
					"stake": "658718014151674744018630428955",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "leadnode.pool.f863973.m0",
					"public_key": "ed25519:CdP6CBFETfWYzrEedmpeqkR6rsJNeT22oUFn2mEDGk5i",
					"stake": "653506381582062491051662734372",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "stakesstone.pool.f863973.m0",
					"public_key": "ed25519:3aAdsKUuzZbjW9hHnmLWFRKwXjmcxsnLNLfNL4gP1wJ8",
					"stake": "650398603386334849496408388535",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "basilisk-stake.pool.f863973.m0",
					"public_key": "ed25519:CFo8vxoEUZoxbs87mGtG8qWUvSBHB91Vc6qWsaEXQ5cY",
					"stake": "648994095757936744237765255329",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "shardlabs.pool.f863973.m0",
					"public_key": "ed25519:DxmhGQZ6oqdxw7qGBvzLuBzE6XQjEh67hk5tt66vhLqL",
					"stake": "646981049290614091876794959528",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "al3c5.pool.f863973.m0",
					"public_key": "ed25519:BoYixTjyBePQ1VYP3s29rZfjtz1FLQ9og4FWZB5UgWCZ",
					"stake": "645886178518400245095079520733",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "dehashed.pool.f863973.m0",
					"public_key": "ed25519:EmPyD1DV9ajWJxjNN8GGACMyhM9w14brwNwYA5WvVaw",
					"stake": "644238111345677828306271061383",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "blockngine.pool.f863973.m0",
					"public_key": "ed25519:CZrTtCP6XkkxWtr3ATnXE8FL6bcG5cHcxfmdRgN7Lm7m",
					"stake": "638958833643365888624964529283",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "machfund.pool.f863973.m0",
					"public_key": "ed25519:G6fJ79oM6taQGhHeQZrg8N36nkCPMEVPyQMHfFT2wAKc",
					"stake": "636787532088125314092433074757",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "projecttent.pool.f863973.m0",
					"public_key": "ed25519:2ueHfYVewchegMmae9bc86ngdD1FWTbxewVb8sr4cABx",
					"stake": "633694652781987662922783076651",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "grassets.pool.f863973.m0",
					"public_key": "ed25519:3S4967Dt1VeeKrwBdTTR5tFEUFSwh17hEFLATRmtUNYV",
					"stake": "631567437549379612196957449611",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "bflame.pool.f863973.m0",
					"public_key": "ed25519:4uYM5RXgR9D6VAGKHgQTVNLEmCgMVX7PzpBstT92Me6R",
					"stake": "625988024445101871042449039125",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "shurik.pool.f863973.m0",
					"public_key": "ed25519:9zEn7DVpvQDxWdj5jSgrqJzqsLo8T9Wv37t83NXBiWi6",
					"stake": "624325578143542580754330954208",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "dsrvlabs.pool.f863973.m0",
					"public_key": "ed25519:61ei2efmmLkeDR1CG6JDEC2U3oZCUuC2K1X16Vmxrud9",
					"stake": "622304298566637122397646551665",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "zetsi.pool.f863973.m0",
					"public_key": "ed25519:6rYx5w1Z2pw46NBHv6Wo4JEUMNtqnDGqPaHT4wm15YRw",
					"stake": "620560050906354607466782420731",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "cryptoblossom-1.pool.f863973.m0",
					"public_key": "ed25519:DdHAHE58vzuouzHg36FvQ24ePGgnyUCQUBKhz9SCy6qf",
					"stake": "606544800393703699937521315458",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "n0ok.pool.f863973.m0",
					"public_key": "ed25519:D6Gq2RpUoDUojmE2vLpqQzuZwYmFPW6rMcXPrwRYhqN8",
					"stake": "602778610223951252559992199415",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "chelovek_iz_naroda.pool.f863973.m0",
					"public_key": "ed25519:89aWsXXytjAZxyefXuGN73efnM9ugKTjPEGV4hDco8AZ",
					"stake": "600914576673095571417994502276",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "lavenderfive.pool.f863973.m0",
					"public_key": "ed25519:AzwAiLDqprZKpDjhsH7dfyvFdfSasmPTjuJUAHfX1Pg4",
					"stake": "594605611374614570572041994142",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "latenthero.pool.f863973.m0",
					"public_key": "ed25519:EQqmjRNouRKhwGL7Hnp3vcbDywg2Boj6to2gmnXybhEM",
					"stake": "579738102423767165960094987835",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "tayang.pool.f863973.m0",
					"public_key": "ed25519:G9XWX55MfWEpT84ckcsJxVTKeZK4WqBGJX3xVpnPB5vv",
					"stake": "570757964693373358771685857916",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "ou812.pool.f863973.m0",
					"public_key": "ed25519:GStYiYQyK1chqTLqFfv8Wx4jVzTRgt1rtdvitCqhziY2",
					"stake": "558769226033905882589378147864",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "smcvalidator.pool.f863973.m0",
					"public_key": "ed25519:pG4LYsyoAa8yWYG9nsTQ5yBcwke51i3VqeRcMVbE9Q7",
					"stake": "555347184321340222996115152098",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "everstake.pool.f863973.m0",
					"public_key": "ed25519:4LDN8tZUTRRc4siGmYCPA67tRyxStACDchdGDZYKdFsw",
					"stake": "554331913041459942528151353385",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "infiniteloop.pool.f863973.m0",
					"public_key": "ed25519:2fbiLqksH5viWXYoteyfKP9qQawkRKw4YogRFcvG3Z7f",
					"stake": "546016740507123799527724438685",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "rossi-validator.pool.f863973.m0",
					"public_key": "ed25519:2eRx2c3KX9wFd3EzuuajFQoSxRTKDqSbxcF13LfkrxCR",
					"stake": "545396796409192203397902952472",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "luckycore.pool.f863973.m0",
					"public_key": "ed25519:6QkPr74dv2v6iMfEVku7RzfKUbFjiSuEFUsLcSRwiiMQ",
					"stake": "522826575280340517682757884968",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "lusienda.pool.f863973.m0",
					"public_key": "ed25519:HdQb2HEiaMgvUdemTt5rkrFbxTpzZyELvg1Vov4LQAGU",
					"stake": "516244171451174048829622136255",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "ino.pool.f863973.m0",
					"public_key": "ed25519:B75h2eqpaMgh6WkAvgnz2FsEC9s5TwVx7zwTjqXKfRs6",
					"stake": "495064502073743765926263103288",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "stakenear.pool.f863973.m0",
					"public_key": "ed25519:Bs7MLvUxXqGkvXJynvMgAGTPp9ou5Dr9x5FY9AiPkQxd",
					"stake": "492251847899939907563159115931",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "nnc.pool.f863973.m0",
					"public_key": "ed25519:98XDErBWqUXhUPW1UaZA6Vx57dFkC2YhMPYEY3BnZQDb",
					"stake": "406164452289351907815075544107",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "kiln.pool.f863973.m0",
					"public_key": "ed25519:Bq8fe1eUgDRexX2CYDMhMMQBiN13j8vTAVFyTNhEfh1W",
					"stake": "98329554883070789245683528853",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "nodemeister.pool.f863973.m0",
					"public_key": "ed25519:85EMyaNGMFuHK2RDH7KHno6fVYBR6iykUXHPPmFTGuTB",
					"stake": "47688416879725069570803688249",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "nala.pool.f863973.m0",
					"public_key": "ed25519:Fzwndob2h5PFdEuwo9eRFJV3BLLurcNaw2SGob5rMPEn",
					"stake": "45372459084177333569120400462",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "happystake.pool.f863973.m0",
					"public_key": "ed25519:3APqZiwzeZLzgfkJyGGTfepDYHA2d8NF1wZi4mCpZnaJ",
					"stake": "44684743204549745857178267079",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "mateennala.pool.f863973.m0",
					"public_key": "ed25519:9kNpQKUKzhc1AiFSEoZcTNapTnywjbXBPngH3EDpD1tw",
					"stake": "40266019415355255243800000000",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "wolfedge-capital-testnet.pool.f863973.m0",
					"public_key": "ed25519:CQEMcPQz6sqhAgoBm9ka9UeVcXj5NpNpRtDYYGkPggvg",
					"stake": "38083427530274718097464129102",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "jstaking.pool.f863973.m0",
					"public_key": "ed25519:fui1E5XwnAWGYDBSQ3168aDfsW1KDFH8A7nBHvZiqGv",
					"stake": "36368377863728647731651257216",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "dariya.pool.f863973.m0",
					"public_key": "ed25519:A5Rx38TsNKWXzF5o18HpaRrPeBzv3riqur51bqhU1Qbp",
					"stake": "36202019246652427577988753829",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "4ire-pool.pool.f863973.m0",
					"public_key": "ed25519:EWPSvYN9pGPMmCLjVxx96stWdqksXNSGnfnuWYn9iiE5",
					"stake": "34380460373758324938162394432",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "lionstake.pool.f863973.m0",
					"public_key": "ed25519:Fy6quR4nBhrEnDyEuPWoAdBP5tzNbuEZsEd91Q5pQnXB",
					"stake": "34139239586235442934772999927",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "zentriav2.factory.colorpalette.testnet",
					"public_key": "ed25519:4rCwSFzJ2e6suD5Yi7pgLidcAJ8Zt9BXieLzVedJDwmE",
					"stake": "31944936095979849504506037787",
					"validator_stake_struct_version": "V1"
				},
				{
					"account_id": "lastnode.pool.f863973.m0",
					"public_key": "ed25519:811gesxXYdYeThry96ZiWn8chgWYNyreiScMkmxg4U9u",
					"stake": "24489695334884067374091015552",
					"validator_stake_struct_version": "V1"
				}
			],
			"prev_block_hash": "H8sRpqgZcW2xzQgKKbmCoTszfz8tsKcPxcZMbNDSNJRV"
		},
		"id": "idontcare"
	}`

	type response struct {
		Result types.LightClientBlockViewJson `json:"result"`
	}

	var r response
	err := json.Unmarshal([]byte(payload), &r)
	if err != nil {
		log.Fatal(err)
	}
	log.Println(r.Result)

}

func TestUnmarshalLightClientBlockViewInnerLite(t *testing.T) {
	payload := `{"inner_lite": {
		"block_merkle_root": "3aTamyUKPYRxKYP3oHaNhJ7LiaKikuBmU1MA5VK6zk5S",
		"epoch_id": "Ad5SqwwdfwdynaXtuzoSzMgjm6m9oZPbxeQf442hP2G2",
		"height": 87002986,
		"next_bp_hash": "2VQp3KoxaSA71PQ13BgBaM7ZuykjKtraSECWqv9GBv3y",
		"next_epoch_id": "2anw7wLLXcUfj3SjK4BBz6K6TNBFM9M1qY9PwmNeo6TU",
		"outcome_root": "CrgUswudNyDeUttTspjxE7RWE88bteAjf9Zz23DvizUa",
		"prev_state_root": "Fk58zL7meg9LrFB8XRheH3rYJxAKsKFxUzXxe9aShJNp",
		"timestamp": 1649382016925706604,
		"timestamp_nanosec": "1649382016925706604"
	}}`

	type result struct {
		InnerLite types.BlockHeaderInnerLiteViewJson `json:"inner_lite"`
	}

	var r result
	err := json.Unmarshal([]byte(payload), &r)
	if err != nil {
		log.Fatal(err)
	}
	assert.Equal(t, "Ad5SqwwdfwdynaXtuzoSzMgjm6m9oZPbxeQf442hP2G2", r.InnerLite.EpochId)
}
