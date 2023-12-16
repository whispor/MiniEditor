<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import {ref} from 'vue';

const Msg = ref('')
const Num = ref(0)

async function hahaVue() {
    Num.value += 1
    let args = "你说啥？"+ Num.value
    Msg.value = await invoke("haha", { text: args });
}
</script>

<template>
    <div class="LayoutContainer">
        这是layout，哈哈...
        {{ Msg }}
        <p>
            <el-button @Click="hahaVue">按钮</el-button>
        </p>
        <!-- <TinyEditor></TinyEditor> -->
    </div>
</template>

<style scss>

</style>