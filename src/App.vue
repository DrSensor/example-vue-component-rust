<template>
  <div id="app">
    <div class="vertical">
      <input type="range" name="y" v-model.number="y" />
    </div>
    <input type="range" name="x" v-model.number="x" />
    <Calculator class="center" :a="x" :b="y" :operation="selected">
      <select v-model="selected">
        <option value="add">add</option>
        <option value="substract">substract</option>
        <option value="multiply">multiply</option>
        <option value="divide">divide</option>
        <option value="power">power</option>
        <option value="remainder">remainder</option>
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
}
</script>

<style scoped>
.center {
  margin-top: -15%;
  margin-left: 15%;
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
