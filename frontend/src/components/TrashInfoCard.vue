<template>
    <div>
        <el-card :style="{width: '400px'}">
            <div>Own by : {{tracker.owner}}</div>
            <div>location: {{tracker.location}}</div>
            <div>weight: {{tracker.weight}}</div>
            <button @click="displayPriceSetting = true">
                Ask to buy
                <i class="el-icon-shopping-bag-1"></i>
            </button>
        </el-card>
        <purchase-info-dialog
                :display-price-setting="displayPriceSetting"
        />
    </div>
</template>
<script>
    import PurchaseInfoDialog from "./PurchaseInfoDialog";

    export default {
        name: 'trash-info-card',
        components: {PurchaseInfoDialog},
        data() {
            return {
                displayPriceSetting: false
            }
        },
        props: {
            tracker: {}
        }
    }
</script>
