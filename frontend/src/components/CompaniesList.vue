<template>
    <el-card v-loading="loadingCompanies" :style="{width: '400px'}">
        <el-button @click="$emit('load-companies')"><i class="el-icon-refresh"></i></el-button>
        <el-card v-for="company in companies" :key="company.name">
            Name: <p>{{company.name}}</p>
            Description: <p>{{company.description}}</p>
            <el-button @click="()=>{this.viewInfo(company)}">View info</el-button>
        </el-card>
        <company-info-dialog :display-info="displayInfo" :selected-company="selectedCompany"/>
    </el-card>


</template>
<script>
    import CompanyInfoDialog from "./CompanyInfoDialog";

    export default {
        name: 'companies-list',
        components: {CompanyInfoDialog},
        data() {
            return {
                displayInfo: false,
                selectedCompany: {}
            }
        },
        methods: {
            viewInfo(company) {
                this.selectedCompany = company;
                this.displayInfo = true;
            }
        },
        props: {
            companies: {},

            loadingCompanies: {
                type: Boolean,
                default: false,
            },
        }
    }
</script>
