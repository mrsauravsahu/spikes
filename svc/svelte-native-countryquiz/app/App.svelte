<script lang="ts">

  let textFieldValue = "";
  let todos = [];

  const onItemTap = (args) => {
    // console.log(
    //   `Item ${todos[args.index].name} at index: ${args.index} was tapped`,
    // );
  };

  const onButtonTap = () => {
    if (textFieldValue === "") {
      console.log("empty input... exiting."); // Logs the newly added task in the console for debugging.

      return; // Prevents users from entering an empty string.
    }
    console.log("New task added: " + textFieldValue + "."); // Logs the newly added task in the console for debugging.
    todos = [{ name: textFieldValue }, ...todos]; // Adds tasks in the ToDo array. Newly added tasks are immediately shown on the screen.
    textFieldValue = ""; // Clears the text field so that users can start adding new tasks immediately.
  };
</script>

<gridLayout columns="*,120" rows="70,*">
  <!-- Configures the text field and ensures that pressing Return on the keyboard
            produces the same result as tapping the button. -->
  <textField
    col="0"
    row="0"
    bind:text={textFieldValue}
    hint="Type todo to complete..."
    editable="true"
    on:returnPress={onButtonTap}
  />
  <button col="1" row="0" text="Add todo" on:tap={onButtonTap} />
  <scrollView row="1">
    <stackLayout>
      {#each todos as item}
        <label
          class="todo-item"
          text={item.name}
          textWrap="true"
          on:tap={onItemTap}
        />
      {/each}
    </stackLayout>
  </scrollView>
</gridLayout>

<style>
  .todo-item {
    margin: 48px;
  }
</style>
