<template>
    <el-card>
        <div v-for="(request, index) in requests" :key="index">
            <div>
                {{request.sender}}
                {{request.trashId}}
                {{request.price}}
                {{request.intend}}
                <el-button>
                    Accept
                </el-button>
                <el-button>
                    refuse
                </el-button>
            </div>
        </div>
    </el-card>
</template>
<script>
    export default {
        name: 'request-list-card',
        props: {
            requests: {}
        }
    }
</script>
