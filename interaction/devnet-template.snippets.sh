BYTECODE=../output/jex-sc-farm.wasm
PROXY=https://devnet-gateway.multiversx.com
SC_ADDRESS=$(mxpy data load --key=address-devnet-template)
CHAIN=D
SCRIPT_DIR=$(dirname $0)

source "${SCRIPT_DIR}/_common.snippets.sh"

deploy() {
    echo 'You are about to deploy SC on devnet (Ctrl-C to abort)'
    read answer

    mxpy contract deploy --bytecode=${BYTECODE} --metadata-payable \
        --keyfile=${1} --gas-limit=50000000 --outfile="deploy-devnet.interaction.json" \
        --arguments "0x" "0x" \
        --proxy=${PROXY} --chain=${CHAIN} --recall-nonce --send || return

    SC_ADDRESS=$(cat deploy-devnet.interaction.json | jq -r .contractAddress)

    mxpy data store --key=address-devnet-template --value=${SC_ADDRESS}

    echo ""
    echo "Smart contract address: ${SC_ADDRESS}"
}

upgrade() {
    echo 'You are about to upgrade current SC on devnet (Ctrl-C to abort)'
    read answer

    mxpy contract upgrade --bytecode=${BYTECODE} --metadata-payable \
        --keyfile=${1} --gas-limit=50000000 --outfile="deploy-devnet.interaction.json" \
        --proxy=${PROXY} --chain=${CHAIN} --recall-nonce --send ${SC_ADDRESS} || return

    echo ""
    echo "Smart contract upgraded: ${SC_ADDRESS}"
}

CMD=$1
shift

$CMD $*
