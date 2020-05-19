<template>
    <el-card v-loading="registering" :style="{width: '400px'}">
        <h1>Become a tracker company</h1>
        <p>Registering give you access to the list of tracking company and the trash the want to get rid of</p>
        <el-input placeholder="Name" v-model="registerForm.name" class="form-input">

        </el-input>
        <el-input placeholder="Description" v-model="registerForm.description"  class="form-input">

        </el-input>
        <el-input placeholder="Contact" v-model="registerForm.contact"  class="form-input">

        </el-input>
        <el-input placeholder="address" v-model="registerForm.address"  class="form-input">

        </el-input>
        <el-button @click="$emit('register', registerForm)"  class="form-input">
            Register
        </el-button>
    </el-card>
</template>
<script>
    export default {
        name: 'register-card',
        data(){
            return{
                registerForm: {
                    address: "",
                    contact: "",
                    description: "",
                    name: "",
                },
            }
        },
        props: {

            registering: {
                type: Boolean,
                required: true
            }
        }
    }
</script>
<style scoped>
    .form-input {
        padding-bottom: .4em;
    }
</style>
