<template>
    <el-dialog title="Purchase information" :visible.sync="displayPriceSetting" v-if="displayPriceSetting">
        What do you intend to do with this payload
        <el-input type="textarea" v-model="processInformation"></el-input>
        How much are you willing to pay for this trash :
        <el-input-number v-model="price"></el-input-number>
        <el-button @click="confirm" :disabled="processInformation.length > 0">Confirm</el-button>
    </el-dialog>
</template>
<script>
    export default {
        name: 'purchase-info-dialog',
        data() {
            return {
                price: 0,
                processInformation: "",
            }
        },
        methods: {
            confirm() {
                this.$emit('confirm', {price: this.price, process: this.processInformation})
            }
        },
        props: {
            displayPriceSetting: {
                type: Boolean,
                required: true
            }
        }
    }
</script>
