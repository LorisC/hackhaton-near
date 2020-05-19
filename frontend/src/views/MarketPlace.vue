<template>
    <el-card>
        <div class="table">
            <div class="table-header">
                <div class="table-title">
                    TrackerId
                </div>
                <div class="table-title">
                    OwnerId
                </div>
                <div class="table-title">
                    Type
                </div>
                <div class="table-title">
                    Weight
                </div>
                <div class="table-title">
                    Location
                </div>

            </div>
            <div class="table-item" v-for="tracker in trackers" :key="tracker.tracker_id">
                <div class="table-item-value">
                    {{tracker.tracker_id}}
                </div>
                <div class="table-item-value">
                    {{tracker.owner}}
                </div>
                <div class="table-item-value">
                    {{tracker.type}}
                </div>
                <div class="table-item-value">
                    {{tracker.weight}}
                </div>
                <div class="table-item-value">
                    {{tracker.location}}
                </div>
                <el-button @click="()=>{requestBuy(tracker)}">Buy</el-button>
            </div>
        </div>
        <purchase-info-dialog :display-price-setting="displayPurchaseDialog" @confirm="onPurchaseConfirmation">

        </purchase-info-dialog>
    </el-card>

</template>

<script>
    import PurchaseInfoDialog from "../components/PurchaseInfoDialog";

    export default {
        name: "MarketPlace",
        components: {PurchaseInfoDialog},
        data() {
            return {
                displayPurchaseDialog: false,
                selectedTrash: null
            }
        },
        computed: {
            trackers() {
                const trackers = [];
                for (let tracker_id in this.$store.state.trackers) {
                    trackers.push(this.$store.state.trackers[tracker_id])
                }
                return trackers;
            }
        },
        methods: {
            requestBuy(e) {
                this.displayPurchaseDialog = true;
                this.selectedTrash = e;
            },
            onPurchaseConfirmation(e){
                console.log(e);
                this.displayPurchaseDialog = false;

            }
        },
        mounted() {
        }
    }
</script>

<style scoped>
    .table {
        display: grid;
        row-gap: .5em;
    }

    .table-header {
        display: grid;
        grid-template-columns: 20% 17.5% 17.5% 17.5% 17.5%;
    }

    .table-item {
        display: grid;
        grid-template-columns: 20% 17.5% 17.5% 17.5% 17.5% 10%;
    }
</style>
