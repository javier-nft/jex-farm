##
# Info
##

echo "Proxy: ${PROXY}"
echo "SC address: ${SC_ADDRESS:-Not deployed}"

##
# Owner endpoints
##

fund() {
    read -p 'Rewards token ID: ' REWARDS_TOKEN_ID
    read -p "Amount (in weis - no float): " AMOUNT

    REWARDS_TOKEN_ID="0x$(echo -n "${REWARDS_TOKEN_ID}" | xxd -ps)"
    METHOD="0x$(echo -n "fund" | xxd -ps)"

    mxpy contract call ${SC_ADDRESS} --recall-nonce --keyfile=${KEYFILE} --gas-limit=5000000 \
        --function="ESDTTransfer" --arguments ${REWARDS_TOKEN_ID} ${AMOUNT} ${METHOD} \
        --proxy=${PROXY} --chain=${CHAIN} --send || return
}

setRewardsDuration() {
    read -p "Duration (sec): " DURATION
    mxpy contract call ${SC_ADDRESS} --recall-nonce --keyfile=${KEYFILE} --gas-limit=4000000 \
        --function="setRewardsDuration" \
        --arguments ${DURATION} \
        --proxy=${PROXY} --chain=${CHAIN} --send || return
}

terminate() {
    LIMIT=${1:-100}
    GAS_LIMIT=$((5000000 + 500000 * LIMIT))

    mxpy contract call ${SC_ADDRESS} --recall-nonce --keyfile=${KEYFILE} --gas-limit=${GAS_LIMIT} \
        --function="terminate" \
        --arguments ${LIMIT} \
        --proxy=${PROXY} --chain=${CHAIN} --send || return
}

##
# Views
##

getAllStakers() {
    SC_ADDRESS=erd1qqqqqqqqqqqqqpgqy2wsj4exvsjwhdv4xxvs2dywn6sc6pl66avsfz733x
    FROM=${1:-0}
    SIZE=${2:-10}
    mxpy contract query ${SC_ADDRESS} \
        --function "getAllStakers" --arguments "${FROM}" "${SIZE}" \
        --proxy=${PROXY} | jq -r .[].hex | while read a; do mxpy wallet bech32 --encode $a; done
}

getBalanceOf() {
    SC_ADDRESS=erd1qqqqqqqqqqqqqpgqy2wsj4exvsjwhdv4xxvs2dywn6sc6pl66avsfz733x
    read -p "Address: " ADDRESS
    ADDRESS="0x$(mxpy wallet bech32 --decode ${ADDRESS})"
    
    mxpy contract query ${SC_ADDRESS} \
        --function "getBalanceOf" --arguments "${ADDRESS}" \
        --proxy=${PROXY}
}

getFinishAt() {
    mxpy contract query ${SC_ADDRESS} --function "getFinishAt" --proxy=${PROXY}
}

getPendingRewards() {
    read -p "Address: " ADDRESS
    ADDRESS="0x$(mxpy wallet bech32 --decode ${ADDRESS})"
    
    mxpy contract query ${SC_ADDRESS} \
        --function "getPendingRewards" --arguments "${ADDRESS}" \
        --proxy=${PROXY}
}

getRewardsDuration() {
    mxpy contract query ${SC_ADDRESS} --function "getRewardsDuration" --proxy=${PROXY}
}

getRewardsToken() {
    mxpy contract query ${SC_ADDRESS} --function "getRewardsToken" --proxy=${PROXY}
}

getRewardPerSecond() {
    mxpy contract query ${SC_ADDRESS} --function "getRewardPerSecond" --proxy=${PROXY}
}

getStakingToken() {
    mxpy contract query ${SC_ADDRESS} --function "getStakingToken" --proxy=${PROXY}
}

getStatus() {
    mxpy contract query ${SC_ADDRESS} --function "getStatus" --proxy=${PROXY}
}

getTotalStaked() {
    mxpy contract query ${SC_ADDRESS} --function "getTotalStaked" --proxy=${PROXY}
}

##
# Public endpoints
##

claim() {
    mxpy contract call ${SC_ADDRESS} --recall-nonce \
        --pem=$1 --gas-limit=6000000 \
        --function="claim" \
        --proxy=${PROXY} --chain=${CHAIN} --send || return
}

exit() {
    mxpy contract call ${SC_ADDRESS} --recall-nonce \
        --pem=$1 --gas-limit=8000000 \
        --function="exit" \
        --proxy=${PROXY} --chain=${CHAIN} --send || return
}

stake() {
    read -p "Token identifier: " TOKEN_IDENTIFIER
    read -p "Amount (in weis - no float): " AMOUNT

    TOKEN_IDENTIFIER="0x$(echo -n "${TOKEN_IDENTIFIER}" | xxd -ps)"
    METHOD="0x$(echo -n "stake" | xxd -ps)"

    mxpy contract call ${SC_ADDRESS} --recall-nonce --pem=$1 --gas-limit=6000000 \
        --function="ESDTTransfer" --arguments ${TOKEN_IDENTIFIER} ${AMOUNT} ${METHOD} \
        --proxy=${PROXY} --chain=${CHAIN} --send || return
}

withdraw() {
    read -p "Amount (in weis - no float): " AMOUNT

    mxpy contract call ${SC_ADDRESS} --recall-nonce --pem=$1 --gas-limit=6000000 \
        --function="withdraw" --arguments ${AMOUNT} \
        --proxy=${PROXY} --chain=${CHAIN} --send || return
}
