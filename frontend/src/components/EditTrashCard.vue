<template>
    <el-card :style="{width: '400px'}">
        <div v-if="!displayEditTrashLocation">
            <div>location: {{tracker.location}}</div>
            <i class="el-icon-edit" @click="displayEditTrashLocation = true"></i>
        </div>
        <div v-else>
            <el-input :placeholder="tracker.location"></el-input>
            <i class="el-icon-check" @click="confirmLocationEdition"></i>
            <i class="el-icon-close" @click="displayEditTrashLocation = false"></i>
        </div>
        <div v-if="!displayEditTrashWeight">
            <div>weight: {{tracker.weight}}</div>
            <i class="el-icon-edit" @click="displayEditTrashWeight = true"></i>
        </div>
        <div v-else>
            Please explain the process you used to reduce the weight of the trash.
            <el-input type="texteara" placeholder="process">

            </el-input>
            <i class="el-icon-check" @click="confirmWeightEdition"></i>
            <i class="el-icon-close" @click="displayEditTrashWeight = false"></i>

        </div>
        <button @click="confirmTrashEdition">
            Edit
        </button>
    </el-card>
</template>
<script>
    export default {
        name: 'edit-trash-card',
        props: {
            confirmLocationEdition: {},
            confirmTrashEdition: {},
            confirmWeightEdition: {},
            displayEditTrashLocation: {},
            displayEditTrashWeight: {},
            tracker: {}
        }
    }
</script>