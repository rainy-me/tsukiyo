<template>
  <section id="menu" class="container mx-auto my-32">
    <SectionTitle text="メニュー" />
    <div class="flex lg:flex-row flex-col relative z-20 lg:mb-64">
      <div class="intro w-1/2 p-16 text-2xl">肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ</div>
      <img class="lg:w-2/5 w-full lg:order-none order-first" src="/assets/DSC08492.jpeg" />
    </div>
    <div class="flex lg:flex-row flex-col relative z-20 lg:mb-64">
      <div class="intro w-1/2 p-16 text-2xl">肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ</div>
      <img class="lg:w-2/5 w-full lg:order-none order-first" src="/assets/DSC08493.jpeg" />
    </div>
    <div class="lg:flex block relative z-20 lg:mb-64">
      <img class="lg:w-3/5 w-full z-0" src="/assets/DSC08494.jpeg" />
      <div
        class="intro right w-1/2 p-16 text-2xl"
      >肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ肉料理だよ</div>
    </div>
    <div class="lg:flex p-16 recommend">
      <RecommendItem title="Title" text="testtesttest" />
      <RecommendItem title="Title" text="testtesttest" />
      <RecommendItem title="Title" text="testtesttest" />
    </div>
  </section>
</template>

<script setup lang="ts">
import SectionTitle from "./SectionTitle.vue";
import RecommendItem from "./RecommendItem.vue";
</script>

<style scoped>
.intro {
  position: relative;
  top: 200px;
  left: 100px;
  display: block;
  height: 300px;
  background-color: wheat;
}
.intro::after {
  content: "";
  display: block;
  width: 100%;
  height: 300px;
  background-color: transparent;
  border: 1px solid wheat;
  position: absolute;
  top: 30px;
  left: -30px;
  z-index: 10;
}
.right {
  top: 300px;
  left: -100px;
}
.right::after {
  top: -30px;
}
.recommend {
  border: 1px solid wheat;
}
@media (max-width: 1024px) {
  .intro {
    width: 100%;
    top: 0;
    left: 0;
    margin: 10px 0;
  }
  .intro::after {
    display: none;
  }
}
</style>