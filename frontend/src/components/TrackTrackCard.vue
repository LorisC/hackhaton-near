<template>
    <el-card v-loading="trackingTrash" :style="{width: '400px'}">
        <h1>Track some trash</h1>
        <el-input placeholder="Location" v-model="trackingForm.location">
        </el-input>
        <el-input placeholder="type" v-model="trackingForm.type">
        </el-input>

        Weight:
        <el-input-number label="weight" placeholder="weight" v-model="trackingForm.weight">
        </el-input-number>
        <br/>
        <el-button @click="$emit('track', trackingForm)">
            Track
        </el-button>
    </el-card>
</template>
<script>
    export default {
        name: 'track-track-card',
        data(){
            return {
                trackingForm: {
                    type: "",
                    location: "",
                    weight: 0
                },
            }
        },
        props: {
            trackingTrash: {
                type: Boolean,
                default: false,
            }
        }
    }
</script>
