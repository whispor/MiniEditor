<script setup lang="ts" name="TinyEditor">
import 'tinymce/themes/silver'
import 'tinymce/icons/default/icons'
import 'tinymce/themes/silver/theme'
import 'tinymce/models/dom/model'
// import tinymce from 'tinymce/tinymce'
import Editor from '@tinymce/tinymce-vue';
import { reactive, ref } from 'vue';

const content = ref('')
const apiKey = '4m8d8v2dod7l9cnux1uz0kys8hkv8ymaagaswlnibc7n4dx1'
const init = reactive({
    language: "zh_CN",
    placeholder: "在这里输入文字",
    min_width: 300,
    min_height: 200,
    height: 400,
    resize: "both",
    branding: false, //tiny技术支持信息是否显示
    statusbar: true,
    elementpath: false,
    font_size_formats: '11px 12px 14px 16px 18px 24px 36px 48px',
    font_family_formats: "微软雅黑=Microsoft YaHei,Helvetica Neue,PingFang SC,sans-serif;苹果苹方=PingFang SC,Microsoft YaHei,sans-serif;宋体=simsun,serif;仿宋体=FangSong,serif;黑体=SimHei,sans-serif;Arial=arial,helvetica,sans-serif;Arial Black=arial black,avant garde;Book Antiqua=book antiqua,palatino;",
    plugins: "preview searchreplace autolink directionality visualblocks visualchars fullscreen image link media template code codesample table charmap pagebreak nonbreaking anchor insertdatetime advlist lists wordcount autosave emoticons",
    toolbar: [
    "fullscreen undo redo restoredraft | forecolor backcolor bold italic underline strikethrough link anchor | alignleft aligncenter alignright alignjustify outdent indent | bullist numlist | blockquote subscript superscript removeformat ",
    "fontfamily fontsize styles | table image axupimgs media emoticons charmap hr pagebreak insertdatetime  selectall visualblocks searchreplace | code print preview | indent2em lineheight formatpainter ",
    "testBtn"
  ],
})
</script>

<template>
    <div class="TinyEditor">
        <Editor v-model="content" :api-key="apiKey" :init="init" />
        hahahha，这是编辑页
    </div>
</template>