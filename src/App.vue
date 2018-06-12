<template>
  <div id="app">
    <div class="vertical">
      <input type="range" name="y" v-model.number="y" />
    </div>
    <select class="title" v-model="operation">
        <option value="arithmatic">arithmatic</option>
        <option value="algebra">algebra (matrix 2x2 diagonal operation)</option>
    </select>
    <input type="range" name="x" v-model.number="x" />
    <Calculator class="center" :a="x" :b="y" :operation="selected" :module="operation">
      <select v-model="selected">
        <option value="add">add</option>
        <option value="substract">substract</option>
        <option value="multiply">multiply</option>
        <option v-if="operation === 'algebra'" value="dot">dot</option>
        <option v-if="operation === 'algebra'" value="tensor">tensor</option>
        <option v-if="operation === 'arithmatic'" value="divide">divide</option>
        <option v-if="operation === 'arithmatic'" value="power">power</option>
        <option v-if="operation === 'arithmatic'" value="remainder">remainder</option>
      </select>
    </Calculator>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator'
import Calculator from './components/Calculator.vue'

@Component({ components: { Calculator } })
export default class App extends Vue {
  x = 0
  y = 0
  selected = 'add'
  operation = 'arithmatic'
}
</script>

<style scoped>
.center {
  margin-top: -75px;
  margin-left: 75px;
}
.title {
  margin-top: -40px;
  position: absolute;
}
.vertical {
  display: inline-block;
  width: 20px;
  height: 150px;
  padding: 0;
}
.vertical input {
  width: 150px;
  height: 20px;
  margin: 0;
  transform-origin: 75px 75px;
  transform: rotate(-90deg);
}
</style>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
