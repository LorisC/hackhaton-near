<template>
    <el-dialog :title="selectedCompany.name + ' Info'" :visible.sync="displayInfo" v-if="displayInfo">
        <p>{{selectedCompany.name}}</p>
        <p>{{selectedCompany.description}}</p>
        <p>{{selectedCompany.contact}}</p>
        <p>{{selectedCompany.address}}</p>
    </el-dialog>
</template>
<script>
    export default {
        name: 'company-info-dialog',
        props: {
            displayInfo: {},
            selectedCompany: {}
        }
    }
</script>
