
<a name="0x1_big_vector"></a>

# Module `0x1::big_vector`



-  [Struct `BigVector`](#0x1_big_vector_BigVector)
-  [Constants](#@Constants_0)
-  [Function `empty`](#0x1_big_vector_empty)
-  [Function `singleton`](#0x1_big_vector_singleton)
-  [Function `destroy_empty`](#0x1_big_vector_destroy_empty)
-  [Function `borrow`](#0x1_big_vector_borrow)
-  [Function `borrow_mut`](#0x1_big_vector_borrow_mut)
-  [Function `append`](#0x1_big_vector_append)
-  [Function `push_back`](#0x1_big_vector_push_back)
-  [Function `pop_back`](#0x1_big_vector_pop_back)
-  [Function `remove`](#0x1_big_vector_remove)
-  [Function `swap_remove`](#0x1_big_vector_swap_remove)
-  [Function `swap`](#0x1_big_vector_swap)
-  [Function `range_reverse`](#0x1_big_vector_range_reverse)
-  [Function `reverse`](#0x1_big_vector_reverse)
-  [Function `index_of`](#0x1_big_vector_index_of)
-  [Function `contains`](#0x1_big_vector_contains)
-  [Function `length`](#0x1_big_vector_length)
-  [Function `is_empty`](#0x1_big_vector_is_empty)


<pre><code><b>use</b> <a href="debug.md#0x1_debug">0x1::debug</a>;
<b>use</b> <a href="../../move-stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="table_with_length.md#0x1_table_with_length">0x1::table_with_length</a>;
<b>use</b> <a href="../../move-stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a name="0x1_big_vector_BigVector"></a>

## Struct `BigVector`

A scalable vector implementation based on tables where elements are grouped into buckets.
Each bucket has a capacity of <code>bucket_size</code> elements.


<pre><code><b>struct</b> <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>buckets: <a href="table_with_length.md#0x1_table_with_length_TableWithLength">table_with_length::TableWithLength</a>&lt;u64, <a href="../../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;T&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>end_index: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>num_buckets: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>bucket_size: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x1_big_vector_EINDEX_OUT_OF_BOUNDS"></a>

Vector index is out of bounds


<pre><code><b>const</b> <a href="big_vector.md#0x1_big_vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>: u64 = 1;
</code></pre>



<a name="0x1_big_vector_EOUT_OF_CAPACITY"></a>

Vector is full


<pre><code><b>const</b> <a href="big_vector.md#0x1_big_vector_EOUT_OF_CAPACITY">EOUT_OF_CAPACITY</a>: u64 = 2;
</code></pre>



<a name="0x1_big_vector_EVECTOR_NOT_EMPTY"></a>

Cannot destroy a non-empty vector


<pre><code><b>const</b> <a href="big_vector.md#0x1_big_vector_EVECTOR_NOT_EMPTY">EVECTOR_NOT_EMPTY</a>: u64 = 3;
</code></pre>



<a name="0x1_big_vector_empty"></a>

## Function `empty`

Regular Vector API
Create an empty vector.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_empty">empty</a>&lt;T: store&gt;(bucket_size: u64): <a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_empty">empty</a>&lt;T: store&gt;(bucket_size: u64): <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt; {
    <b>assert</b>!(bucket_size &gt; 0, 0);
    <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a> {
        buckets: <a href="table_with_length.md#0x1_table_with_length_new">table_with_length::new</a>(),
        end_index: 0,
        num_buckets: 0,
        bucket_size,
    }
}
</code></pre>



</details>

<a name="0x1_big_vector_singleton"></a>

## Function `singleton`

Create a vector of length 1 containing the passed in element.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_singleton">singleton</a>&lt;T: store&gt;(e: T, bucket_size: u64): <a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_singleton">singleton</a>&lt;T: store&gt;(e: T, bucket_size: u64): <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt; {
    <b>let</b> v = <a href="big_vector.md#0x1_big_vector_empty">empty</a>(bucket_size);
    <a href="big_vector.md#0x1_big_vector_push_back">push_back</a>(&<b>mut</b> v, e);
    v
}
</code></pre>



</details>

<a name="0x1_big_vector_destroy_empty"></a>

## Function `destroy_empty`

Destroy the vector <code>v</code>.
Aborts if <code>v</code> is not empty.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_destroy_empty">destroy_empty</a>&lt;T&gt;(v: <a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_destroy_empty">destroy_empty</a>&lt;T&gt;(v: <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;) {
    <b>assert</b>!(<a href="big_vector.md#0x1_big_vector_is_empty">is_empty</a>(&v), <a href="../../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="big_vector.md#0x1_big_vector_EVECTOR_NOT_EMPTY">EVECTOR_NOT_EMPTY</a>));
    <b>let</b> <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a> { buckets, end_index: _, num_buckets: _, bucket_size: _ } = v;
    <a href="table_with_length.md#0x1_table_with_length_destroy_empty">table_with_length::destroy_empty</a>(buckets);
}
</code></pre>



</details>

<a name="0x1_big_vector_borrow"></a>

## Function `borrow`

Acquire an immutable reference to the <code>i</code>th element of the vector <code>v</code>.
Aborts if <code>i</code> is out of bounds.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_borrow">borrow</a>&lt;T&gt;(v: &<a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;, i: u64): &T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_borrow">borrow</a>&lt;T&gt;(v: &<a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;, i: u64): &T {
    <b>assert</b>!(i &lt; <a href="big_vector.md#0x1_big_vector_length">length</a>(v), <a href="../../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="big_vector.md#0x1_big_vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>));
    <a href="debug.md#0x1_debug_print">debug::print</a>(&i);
    <a href="../../move-stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(<a href="table_with_length.md#0x1_table_with_length_borrow">table_with_length::borrow</a>(&v.buckets, i / v.bucket_size), i % v.bucket_size)
}
</code></pre>



</details>

<a name="0x1_big_vector_borrow_mut"></a>

## Function `borrow_mut`

Return a mutable reference to the <code>i</code>th element in the vector <code>v</code>.
Aborts if <code>i</code> is out of bounds.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_borrow_mut">borrow_mut</a>&lt;T&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;, i: u64): &<b>mut</b> T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_borrow_mut">borrow_mut</a>&lt;T&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;, i: u64): &<b>mut</b> T {
    <b>assert</b>!(i &lt; <a href="big_vector.md#0x1_big_vector_length">length</a>(v), <a href="../../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="big_vector.md#0x1_big_vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>));
    <a href="../../move-stdlib/doc/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(<a href="table_with_length.md#0x1_table_with_length_borrow_mut">table_with_length::borrow_mut</a>(&<b>mut</b> v.buckets, i / v.bucket_size), i % v.bucket_size)
}
</code></pre>



</details>

<a name="0x1_big_vector_append"></a>

## Function `append`



<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_append">append</a>&lt;T: store&gt;(lhs: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;, other: <a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_append">append</a>&lt;T: store&gt;(lhs: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;, other: <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;) {
    <b>let</b> other_len = <a href="big_vector.md#0x1_big_vector_length">length</a>(&other);
    <b>let</b> half_other_len = other_len / 2;
    <b>let</b> i = 0;
    <b>while</b> (i &lt; half_other_len) {
        <a href="big_vector.md#0x1_big_vector_push_back">push_back</a>(lhs, <a href="big_vector.md#0x1_big_vector_swap_remove">swap_remove</a>(&<b>mut</b> other, i));
        i = i + 1;
    };
    <b>while</b> (i &lt; other_len) {
        <a href="big_vector.md#0x1_big_vector_push_back">push_back</a>(lhs, <a href="big_vector.md#0x1_big_vector_pop_back">pop_back</a>(&<b>mut</b> other));
    };
    <a href="big_vector.md#0x1_big_vector_destroy_empty">destroy_empty</a>(other);
}
</code></pre>



</details>

<a name="0x1_big_vector_push_back"></a>

## Function `push_back`

Add element <code>val</code> to the end of the vector <code>v</code>. It grows the buckets when the current buckets are full.
This operation will cost more gas when it adds new bucket.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_push_back">push_back</a>&lt;T: store&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;, val: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_push_back">push_back</a>&lt;T: store&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;, val: T) {
    <b>if</b> (v.end_index == v.num_buckets * v.bucket_size) {
        <a href="table_with_length.md#0x1_table_with_length_add">table_with_length::add</a>(&<b>mut</b> v.buckets, v.num_buckets, <a href="../../move-stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>());
        v.num_buckets = v.num_buckets + 1;
    };
    <a href="debug.md#0x1_debug_print">debug::print</a>(&val);
    <a href="../../move-stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(<a href="table_with_length.md#0x1_table_with_length_borrow_mut">table_with_length::borrow_mut</a>(&<b>mut</b> v.buckets, v.num_buckets - 1), val);
    v.end_index = v.end_index + 1;
}
</code></pre>



</details>

<a name="0x1_big_vector_pop_back"></a>

## Function `pop_back`

Pop an element from the end of vector <code>v</code>. It doesn't shrink the buckets even if they're empty.
Call <code>shrink_to_fit</code> explicity to deallocate empty buckets.
Aborts if <code>v</code> is empty.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_pop_back">pop_back</a>&lt;T&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_pop_back">pop_back</a>&lt;T&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;): T {
    <b>assert</b>!(!<a href="big_vector.md#0x1_big_vector_is_empty">is_empty</a>(v), <a href="../../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="big_vector.md#0x1_big_vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>));
    <b>let</b> last_bucket = <a href="table_with_length.md#0x1_table_with_length_borrow_mut">table_with_length::borrow_mut</a>(&<b>mut</b> v.buckets, v.num_buckets - 1);
    <b>let</b> val = <a href="../../move-stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(last_bucket);
    // Shrink the <a href="table.md#0x1_table">table</a> <b>if</b> the last <a href="../../move-stdlib/doc/vector.md#0x1_vector">vector</a> is empty.
    <b>if</b> (<a href="../../move-stdlib/doc/vector.md#0x1_vector_is_empty">vector::is_empty</a>(last_bucket)) {
        <b>move</b> last_bucket;
        <a href="../../move-stdlib/doc/vector.md#0x1_vector_destroy_empty">vector::destroy_empty</a>(<a href="table_with_length.md#0x1_table_with_length_remove">table_with_length::remove</a>(&<b>mut</b> v.buckets, v.num_buckets - 1));
    };
    v.end_index = v.end_index - 1;
    val
}
</code></pre>



</details>

<a name="0x1_big_vector_remove"></a>

## Function `remove`

Remove the element at index i in the vector v and return the owned value that was previously stored at i in v.
All elements occurring at indices greater than i will be shifted down by 1. Will abort if i is out of bounds.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_remove">remove</a>&lt;T&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;, i: u64): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_remove">remove</a>&lt;T&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;, i: u64): T {
    <b>let</b> len = <a href="big_vector.md#0x1_big_vector_length">length</a>(v);
    <b>assert</b>!(i &lt; len, <a href="../../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="big_vector.md#0x1_big_vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>));
    <b>let</b> val = <a href="big_vector.md#0x1_big_vector_swap_remove">swap_remove</a>(v, i);
    <b>if</b> (i &lt; len - 1) {
        <a href="big_vector.md#0x1_big_vector_range_reverse">range_reverse</a>(v, i + 1, len);
    };
    <a href="big_vector.md#0x1_big_vector_range_reverse">range_reverse</a>(v, i, len);
    val
}
</code></pre>



</details>

<a name="0x1_big_vector_swap_remove"></a>

## Function `swap_remove`

Swap the <code>i</code>th element of the vector <code>v</code> with the last element and then pop the vector.
This is O(1), but does not preserve ordering of elements in the vector.
Aborts if <code>i</code> is out of bounds.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_swap_remove">swap_remove</a>&lt;T&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;, i: u64): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_swap_remove">swap_remove</a>&lt;T&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;, i: u64): T {
    <b>assert</b>!(i &lt; <a href="big_vector.md#0x1_big_vector_length">length</a>(v), <a href="../../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="big_vector.md#0x1_big_vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>));
    <b>let</b> last_val = <a href="big_vector.md#0x1_big_vector_pop_back">pop_back</a>(v);
    // <b>if</b> the requested value is the last one, <b>return</b> it
    <b>if</b> (v.end_index == i + 1) {
        <b>return</b> last_val
    };
    // because the lack of mem::swap, here we swap remove the requested value from the bucket
    // and append the last_val <b>to</b> the bucket then swap the last bucket val back
    <b>let</b> bucket = <a href="table_with_length.md#0x1_table_with_length_borrow_mut">table_with_length::borrow_mut</a>(&<b>mut</b> v.buckets, i / v.bucket_size);
    <b>let</b> bucket_len = <a href="../../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(bucket);
    <b>let</b> val = <a href="../../move-stdlib/doc/vector.md#0x1_vector_swap_remove">vector::swap_remove</a>(bucket, i % v.bucket_size);
    <a href="../../move-stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(bucket, last_val);
    <a href="../../move-stdlib/doc/vector.md#0x1_vector_swap">vector::swap</a>(bucket, i % v.bucket_size, bucket_len - 1);
    val
}
</code></pre>



</details>

<a name="0x1_big_vector_swap"></a>

## Function `swap`

Swap the elements at the i'th and j'th indices in the vector v. Will abort if either of i or j are out of bounds
for v.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_swap">swap</a>&lt;T&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;, i: u64, j: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_swap">swap</a>&lt;T&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;, i: u64, j: u64) {
    <b>assert</b>!(i &lt; <a href="big_vector.md#0x1_big_vector_length">length</a>(v) && j &lt; <a href="big_vector.md#0x1_big_vector_length">length</a>(v), <a href="../../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="big_vector.md#0x1_big_vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>));
    <b>let</b> i_bucket_index = i / v.bucket_size;
    <b>let</b> j_bucket_index = j / v.bucket_size;
    <b>let</b> i_vector_index = i % v.bucket_size;
    <b>let</b> j_vector_index = j % v.bucket_size;
    <b>if</b> (i_bucket_index == j_bucket_index) {
        <a href="../../move-stdlib/doc/vector.md#0x1_vector_swap">vector::swap</a>(<a href="table_with_length.md#0x1_table_with_length_borrow_mut">table_with_length::borrow_mut</a>(&<b>mut</b> v.buckets, i_bucket_index), i_vector_index, j_vector_index);
        <b>return</b>
    };
    <b>let</b> bucket_i = <a href="table_with_length.md#0x1_table_with_length_remove">table_with_length::remove</a>(&<b>mut</b> v.buckets, i_bucket_index);
    <b>let</b> bucket_j = <a href="table_with_length.md#0x1_table_with_length_remove">table_with_length::remove</a>(&<b>mut</b> v.buckets, j_bucket_index);
    <b>let</b> element_i = <a href="../../move-stdlib/doc/vector.md#0x1_vector_swap_remove">vector::swap_remove</a>(&<b>mut</b> bucket_i, i_vector_index);
    <b>let</b> element_j = <a href="../../move-stdlib/doc/vector.md#0x1_vector_swap_remove">vector::swap_remove</a>(&<b>mut</b> bucket_j, j_vector_index);
    <a href="../../move-stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> bucket_i, element_j);
    <a href="../../move-stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> bucket_j, element_i);
    <b>let</b> last_index_in_bucket_i = <a href="../../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&bucket_i) - 1;
    <b>let</b> last_index_in_bucket_j = <a href="../../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&bucket_j) - 1;
    <a href="../../move-stdlib/doc/vector.md#0x1_vector_swap">vector::swap</a>(&<b>mut</b> bucket_i, i_vector_index, last_index_in_bucket_i);
    <a href="../../move-stdlib/doc/vector.md#0x1_vector_swap">vector::swap</a>(&<b>mut</b> bucket_j, j_vector_index, last_index_in_bucket_j);
    <a href="table_with_length.md#0x1_table_with_length_add">table_with_length::add</a>(&<b>mut</b> v.buckets, i_bucket_index, bucket_i);
    <a href="table_with_length.md#0x1_table_with_length_add">table_with_length::add</a>(&<b>mut</b> v.buckets, j_bucket_index, bucket_j);
}
</code></pre>



</details>

<a name="0x1_big_vector_range_reverse"></a>

## Function `range_reverse`

Reverse the order of the elements in range [i, j) of the vector v.


<pre><code><b>fun</b> <a href="big_vector.md#0x1_big_vector_range_reverse">range_reverse</a>&lt;T&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;, i: u64, j: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="big_vector.md#0x1_big_vector_range_reverse">range_reverse</a>&lt;T&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;, i: u64, j: u64) {
    <b>let</b> half_len = (j - i) / 2;
    <b>let</b> k = 0;
    <b>while</b> (k &lt; half_len) {
        <a href="big_vector.md#0x1_big_vector_swap">swap</a>(v, i + k, j - 1 - k);
        k = k + 1;
    }
}
</code></pre>



</details>

<a name="0x1_big_vector_reverse"></a>

## Function `reverse`

Reverse the order of the elements in the vector v in-place.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_reverse">reverse</a>&lt;T&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_reverse">reverse</a>&lt;T&gt;(v: &<b>mut</b> <a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;) {
    <b>let</b> len = <a href="big_vector.md#0x1_big_vector_length">length</a>(v);
    <a href="big_vector.md#0x1_big_vector_range_reverse">range_reverse</a>(v, 0, len);
}
</code></pre>



</details>

<a name="0x1_big_vector_index_of"></a>

## Function `index_of`

Return the index of the first occurrence of an element in v that is equal to e. Returns (true, index) if such an
element was found, and (false, 0) otherwise.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_index_of">index_of</a>&lt;T&gt;(v: &<a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;, val: &T): (bool, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_index_of">index_of</a>&lt;T&gt;(v: &<a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;, val: &T): (bool, u64) {
    <b>let</b> i = 0;
    <b>let</b> len = <a href="big_vector.md#0x1_big_vector_length">length</a>(v);
    <b>while</b> (i &lt; len) {
        <b>if</b> (<a href="big_vector.md#0x1_big_vector_borrow">borrow</a>(v, i) == val) {
            <b>return</b> (<b>true</b>, i)
        };
        i = i + 1;
    };
    (<b>false</b>, 0)
}
</code></pre>



</details>

<a name="0x1_big_vector_contains"></a>

## Function `contains`

Return if an element equal to e exists in the vector v.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_contains">contains</a>&lt;T&gt;(v: &<a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;, val: &T): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_contains">contains</a>&lt;T&gt;(v: &<a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;, val: &T): bool {
    <b>if</b> (<a href="big_vector.md#0x1_big_vector_is_empty">is_empty</a>(v)) <b>return</b> <b>false</b>;
    <b>let</b> (exist, _) = <a href="big_vector.md#0x1_big_vector_index_of">index_of</a>(v, val);
    exist
}
</code></pre>



</details>

<a name="0x1_big_vector_length"></a>

## Function `length`

Return the length of the vector.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_length">length</a>&lt;T&gt;(v: &<a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_length">length</a>&lt;T&gt;(v: &<a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;): u64 {
    v.end_index
}
</code></pre>



</details>

<a name="0x1_big_vector_is_empty"></a>

## Function `is_empty`

Return <code><b>true</b></code> if the vector <code>v</code> has no elements and <code><b>false</b></code> otherwise.


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_is_empty">is_empty</a>&lt;T&gt;(v: &<a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="big_vector.md#0x1_big_vector_is_empty">is_empty</a>&lt;T&gt;(v: &<a href="big_vector.md#0x1_big_vector_BigVector">BigVector</a>&lt;T&gt;): bool {
    <a href="big_vector.md#0x1_big_vector_length">length</a>(v) == 0
}
</code></pre>



</details>


[move-book]: https://move-language.github.io/move/introduction.html
