<script setup lang="ts">
import {FolderOpened,CirclePlus, Pointer, Promotion,Share,Tools,HelpFilled,SwitchFilled,ElemeFilled,MoreFilled,Minus,Close,FullScreen} from '@element-plus/icons-vue'
import {ref} from 'vue'
import { invoke } from "@tauri-apps/api/tauri";

// 无边框，https://www.cnblogs.com/xxcf/articles/17332797.html
import { appWindow } from '@tauri-apps/api/window'

const textLong = ref('hahhahahahhh哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈\n哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈\n哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈\n哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈\n哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈\n哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈\n哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈\n哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈\n哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈\n哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈\n哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈\n哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈\n哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈\n哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈哈')
const numStr = ref(0)

async function OpenFile() {
    textLong.value = await invoke("get_text",{indx: 0, width: 10000})
}
</script>

<template>
    <div class="Editor">
        <el-row class="row-1">
            <div style="margin: 1px 2px 1px 2px; justify-content:flex-start;width: 100%;" >
                <el-card class="MenuCard" :body-style="{ padding: '0px' }" >
                    <div class="ToolsLayout" style="display: flex;justify-content: space-between;" data-tauri-drag-region>
                        <div class="Tools">
                            <el-icon @click="OpenFile"><FolderOpened /></el-icon>
                            <el-icon><ElemeFilled /></el-icon>
                            <el-icon alt="这是个点击Icon"><Pointer /></el-icon>
                            <el-icon><SwitchFilled /></el-icon>
                            <el-icon><HelpFilled /></el-icon>
                            <el-icon><Tools /></el-icon>
                            <el-icon alt="这是个添加Icon"><CirclePlus /></el-icon>
                            <el-icon><Share /></el-icon>
                            <el-icon alt="这是个添加Icon"><CirclePlus /></el-icon>
                            <el-icon><Promotion /></el-icon>
                            <el-icon><MoreFilled /></el-icon>
                        </div>
                        <div class="MinClose" style="flex-wrap: inline;">
                            <el-icon @click="appWindow.minimize()"><Minus /></el-icon>
                            <el-icon @click="appWindow.toggleMaximize()"><FullScreen /></el-icon>
                            <el-icon @click="appWindow.close()"><Close /></el-icon>
                        </div>
                    </div>

                </el-card>
            </div>            
        </el-row>
        <el-row class="row-2">
            <div class="TextRow">
                    <div style="height: 100%; width: 100%; background-color: rgb(14, 12, 10);boxShadow: lighter" >
                        <el-input class="InputText" show-word-limit resize="none"
                            v-model="textLong"
                            :rows="2"
                            type="textarea"
                            placeholder="Please input"
                            large 
                        />
                    </div>
            </div>
        </el-row>
        <el-row class="row-3">
            <div class="footer">
                <span style="padding-left:10px;font-size: 14px;">字数：{{ numStr }}</span>
            </div>
        </el-row>
    </div>
</template>

<style scoped lang="scss">
.Editor{
    position : absolute;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    margin: 0px;
    border:1px solid rgb(166, 165, 165);
    // border-color: rgb(0, 0, 0)    // border-style: ;
    border-radius: 5px;
    width: 100%;
    height: 100%;
    box-sizing: border-box;   //https://www.jianshu.com/p/2843ff850ff6
    overflow: hidden; // 可以去掉右侧scrollbar

    .row-1{
        border: 1px 1px 1px 1px;
    }

    .MenuCard{
        height: 100%;
        width: 100%;
        border-left: 1px;
        border-right: 1rem;
        padding: 0px;

        .el-card{
            padding: 1px;
            width: 100%;


        }
        .el-card__body{
            padding: 0px;
            overflow:hidden;
        }
    }
    .MinClose, .Tools{
        display: flex;
        flex-flow: row wrap;
        justify-content:flex-start;
        border: 1px;
        justify-items:center;

        .el-icon{
            border: 10px;
            padding: 10px;

            
        }
        .el-icon:hover{
            background-color: rgb(218, 217, 217);
            border-radius: 5%;
        }
    }

    .row-2{
        height: 100%;

        .TextRow{
            height: 100%;
            display: flex;
            border-radius: 0px;
            box-sizing: border-box;
            margin: 1px 2px 1px 2px; 
            width: 100%;

            .el-col-24{
                height: 100%;

                .el-textarea__inner{
                    border-radius: 0px;
                }
            }
        }
    }

    .InputText{
        height: 100%;

        :deep(.el-textarea__inner){
            height: 100%;
        }
    }

    .footer{
        display: flex;
        border: 1px 0px 0px 0px;
        height: 20px; 
        width: 100%;
        margin: 1px 2px 1px 2px ;
        background-color: #ffffff;
        border-radius: 0px 0px 5px 5px ;
    }
}

</style>