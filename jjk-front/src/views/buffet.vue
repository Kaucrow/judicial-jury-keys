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
          <FileUpload mode="basic" name="casefile" url="/jjk/tx/upload" accept=".pdf" :maxFileSize="1000000" class="inline-block" :auto="true" @upload="onUpload"/>
        </div>
        <transition name="fade-scale">
          <div v-if="showImage" class="flex flex-col items-center justify-center mt-4">
            <img src="/half.png" alt="Upload Success" class="max-w-full h-auto" />
            <p class="mt-4 text-lg font-semibold text-blue-500">Nah, I'd win</p>
          </div>
        </transition>
      </div>
    </div>
</template>

<style scoped>
.fade-scale-enter-active {
  transition: all 0.6s ease-out;
}

.fade-scale-leave-active {
  transition: all 0.3s ease-in;
}

.fade-scale-enter-from {
  opacity: 0;
  transform: scale(0.8);
}

.fade-scale-leave-to {
  opacity: 0;
  transform: scale(0.9);
}
</style>
