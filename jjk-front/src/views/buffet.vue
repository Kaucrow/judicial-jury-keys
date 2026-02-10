<script setup>
import { ref } from 'vue';
import FileUpload from 'primevue/fileupload';
import Button from 'primevue/button';

const showImage = ref(false);

const onUpload = () => {
  showImage.value = true;
};
</script>

<template>
    <div class="justify-content-center text-center m-5">
    <h1 class="text-3xl font-bold tracking-tight text-heading md:text-4xl">Buffet Office</h1>

  <h2 class="text-lg font-normal text-body lg:text-xl">Upload the case file to send to the prosecutor office</h2>
  <div class="p-4 m-3">
  <div class="p-4 m-3 flex items-center justify-center gap-3">
    <FileUpload mode="basic" name="casefile" url="/upload" accept=".pdf" :maxFileSize="1000000" class="inline-block" :auto="true" @upload="onUpload"/>
    
    </div>
    <div v-if="showImage" class="flex items-center justify-center mt-4">
      <img src="/half.jpeg" alt="Upload Success" class="max-w-full h-auto" />
    </div>
    </div>
  </div>
</template>

<style scoped></style>
