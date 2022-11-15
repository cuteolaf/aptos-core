
<a name="0x1_smart_vector"></a>

# Module `0x1::smart_vector`



-  [Struct `SmartVector`](#0x1_smart_vector_SmartVector)
-  [Constants](#@Constants_0)
-  [Function `size_of_val`](#0x1_smart_vector_size_of_val)
-  [Function `empty`](#0x1_smart_vector_empty)
-  [Function `singleton`](#0x1_smart_vector_singleton)
-  [Function `destroy_empty`](#0x1_smart_vector_destroy_empty)
-  [Function `borrow`](#0x1_smart_vector_borrow)
-  [Function `borrow_mut`](#0x1_smart_vector_borrow_mut)
-  [Function `append`](#0x1_smart_vector_append)
-  [Function `push_back`](#0x1_smart_vector_push_back)
-  [Function `pop_back`](#0x1_smart_vector_pop_back)
-  [Function `remove`](#0x1_smart_vector_remove)
-  [Function `swap_remove`](#0x1_smart_vector_swap_remove)
-  [Function `swap`](#0x1_smart_vector_swap)
-  [Function `reverse`](#0x1_smart_vector_reverse)
-  [Function `index_of`](#0x1_smart_vector_index_of)
-  [Function `contains`](#0x1_smart_vector_contains)
-  [Function `length`](#0x1_smart_vector_length)
-  [Function `is_empty`](#0x1_smart_vector_is_empty)


<pre><code><b>use</b> <a href="../../move-stdlib/doc/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="big_vector.md#0x1_big_vector">0x1::big_vector</a>;
<b>use</b> <a href="../../move-stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="math64.md#0x1_math64">0x1::math64</a>;
<b>use</b> <a href="../../move-stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a name="0x1_smart_vector_SmartVector"></a>

## Struct `SmartVector`

A Scalable vector implementation based on tables, elements are grouped into buckets with <code>bucket_size</code>.
The internal vectors are actually option but it is intentional because Option requires <code>T</code> to be <code>drop</code> but
<code><a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a></code> should not enforece that.


<pre><code><b>struct</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>inline_inner: <a href="../../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;T&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>table_inner: <a href="../../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="big_vector.md#0x1_big_vector_BigVector">big_vector::BigVector</a>&lt;T&gt;&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x1_smart_vector_EINDEX_OUT_OF_BOUNDS"></a>

Vector index is out of bounds


<pre><code><b>const</b> <a href="smart_vector.md#0x1_smart_vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>: u64 = 1;
</code></pre>



<a name="0x1_smart_vector_EOUT_OF_CAPACITY"></a>

Vector is full


<pre><code><b>const</b> <a href="smart_vector.md#0x1_smart_vector_EOUT_OF_CAPACITY">EOUT_OF_CAPACITY</a>: u64 = 2;
</code></pre>



<a name="0x1_smart_vector_EVECTOR_NOT_EMPTY"></a>

Cannot destroy a non-empty vector


<pre><code><b>const</b> <a href="smart_vector.md#0x1_smart_vector_EVECTOR_NOT_EMPTY">EVECTOR_NOT_EMPTY</a>: u64 = 3;
</code></pre>



<a name="0x1_smart_vector_size_of_val"></a>

## Function `size_of_val`



<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_size_of_val">size_of_val</a>&lt;T&gt;(val_ref: &T): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_size_of_val">size_of_val</a>&lt;T&gt;(val_ref: &T): u64 {
    // Return <a href="../../move-stdlib/doc/vector.md#0x1_vector">vector</a> length of vectorized BCS representation.
    <a href="../../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&<a href="../../move-stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(val_ref))
}
</code></pre>



</details>

<a name="0x1_smart_vector_empty"></a>

## Function `empty`

Regular Vector API
Create an empty vector.


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_empty">empty</a>&lt;T: store&gt;(): <a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_empty">empty</a>&lt;T: store&gt;(): <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt; {
    <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a> {
        inline_inner: <a href="../../move-stdlib/doc/vector.md#0x1_vector">vector</a>[],
        table_inner: <a href="../../move-stdlib/doc/vector.md#0x1_vector">vector</a>[],
    }
}
</code></pre>



</details>

<a name="0x1_smart_vector_singleton"></a>

## Function `singleton`



<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_singleton">singleton</a>&lt;T: store&gt;(e: T): <a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_singleton">singleton</a>&lt;T: store&gt;(e: T): <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt; {
    <b>let</b> v = <a href="smart_vector.md#0x1_smart_vector_empty">empty</a>();
    <a href="smart_vector.md#0x1_smart_vector_push_back">push_back</a>(&<b>mut</b> v, e);
    v
}
</code></pre>



</details>

<a name="0x1_smart_vector_destroy_empty"></a>

## Function `destroy_empty`

Destroy the vector <code>v</code>.
Aborts if <code>v</code> is not empty.


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_destroy_empty">destroy_empty</a>&lt;T&gt;(v: <a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_destroy_empty">destroy_empty</a>&lt;T&gt;(v: <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;) {
    <b>assert</b>!(<a href="smart_vector.md#0x1_smart_vector_is_empty">is_empty</a>(&v), <a href="../../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="smart_vector.md#0x1_smart_vector_EVECTOR_NOT_EMPTY">EVECTOR_NOT_EMPTY</a>));
    <b>let</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a> { inline_inner, table_inner} = v;
    <a href="../../move-stdlib/doc/vector.md#0x1_vector_destroy_empty">vector::destroy_empty</a>(inline_inner);
    <a href="../../move-stdlib/doc/vector.md#0x1_vector_destroy_empty">vector::destroy_empty</a>(table_inner);
}
</code></pre>



</details>

<a name="0x1_smart_vector_borrow"></a>

## Function `borrow`

Acquire an immutable reference to the <code>i</code>th element of the vector <code>v</code>.
Aborts if <code>i</code> is out of bounds.


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_borrow">borrow</a>&lt;T&gt;(v: &<a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;, i: u64): &T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_borrow">borrow</a>&lt;T&gt;(v: &<a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;, i: u64): &T {
    <b>assert</b>!(i &lt; <a href="smart_vector.md#0x1_smart_vector_length">length</a>(v), <a href="../../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="smart_vector.md#0x1_smart_vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>));
    <b>let</b> inline_len = <a href="../../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&v.inline_inner);
    <b>if</b> (i &lt; inline_len) {
        <a href="../../move-stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&v.inline_inner, i)
    } <b>else</b> {
        <a href="big_vector.md#0x1_big_vector_borrow">big_vector::borrow</a>(<a href="../../move-stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&v.table_inner, 0), i - inline_len)
    }
}
</code></pre>



</details>

<a name="0x1_smart_vector_borrow_mut"></a>

## Function `borrow_mut`

Return a mutable reference to the <code>i</code>th element in the vector <code>v</code>.
Aborts if <code>i</code> is out of bounds.


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_borrow_mut">borrow_mut</a>&lt;T&gt;(v: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;, i: u64): &<b>mut</b> T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_borrow_mut">borrow_mut</a>&lt;T&gt;(v: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;, i: u64): &<b>mut</b> T {
    <b>assert</b>!(i &lt; <a href="smart_vector.md#0x1_smart_vector_length">length</a>(v), <a href="../../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="smart_vector.md#0x1_smart_vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>));
    <b>let</b> inline_len = <a href="../../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&v.inline_inner);
    <b>if</b> (i &lt; inline_len) {
        <a href="../../move-stdlib/doc/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(&<b>mut</b> v.inline_inner, i)
    } <b>else</b> {
        <a href="big_vector.md#0x1_big_vector_borrow_mut">big_vector::borrow_mut</a>(<a href="../../move-stdlib/doc/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(&<b>mut</b> v.table_inner, 0), i - inline_len)
    }
}
</code></pre>



</details>

<a name="0x1_smart_vector_append"></a>

## Function `append`



<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_append">append</a>&lt;T: store&gt;(lhs: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;, other: <a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_append">append</a>&lt;T: store&gt;(lhs: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;, other: <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;) {
    <b>let</b> half_len = <a href="smart_vector.md#0x1_smart_vector_length">length</a>(&other) / 2;
    <b>let</b> i = 0;
    <b>while</b> (i &lt; half_len) {
        <a href="smart_vector.md#0x1_smart_vector_push_back">push_back</a>(lhs, <a href="smart_vector.md#0x1_smart_vector_swap_remove">swap_remove</a>(&<b>mut</b> other, i));
        i = i + 1;
    };
    <b>while</b> (<a href="smart_vector.md#0x1_smart_vector_length">length</a>(&other) &gt; 0) {
        <a href="smart_vector.md#0x1_smart_vector_push_back">push_back</a>(lhs, <a href="smart_vector.md#0x1_smart_vector_pop_back">pop_back</a>(&<b>mut</b> other));
    };
    <a href="smart_vector.md#0x1_smart_vector_destroy_empty">destroy_empty</a>(other);
}
</code></pre>



</details>

<a name="0x1_smart_vector_push_back"></a>

## Function `push_back`

Add element <code>val</code> to the end of the vector <code>v</code>. It grows the buckets when the current buckets are full.
This operation will cost more gas when it adds new bucket.


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_push_back">push_back</a>&lt;T: store&gt;(v: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;, val: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_push_back">push_back</a>&lt;T: store&gt;(v: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;, val: T) {
    <b>let</b> len = <a href="smart_vector.md#0x1_smart_vector_length">length</a>(v);
    <b>let</b> inline_len = <a href="../../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&v.inline_inner);
    <b>let</b> val_size = <a href="smart_vector.md#0x1_smart_vector_size_of_val">size_of_val</a>(&val);
    <b>if</b> (len == inline_len) {
        <b>if</b> (val_size * (inline_len + 1) &lt; 150 /* magic number */) {
            <b>assert</b>!(<a href="../../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&v.table_inner) == 0, 123);
            <a href="../../move-stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> v.inline_inner, val);
            <b>return</b>
        };
        <b>let</b> estimated_avg_size = (<a href="smart_vector.md#0x1_smart_vector_size_of_val">size_of_val</a>(&v.inline_inner) + val_size) / (inline_len + 1);
        <b>let</b> bucket_size = max(1024 /* free_write_quota */ / estimated_avg_size, 1);
        <a href="../../move-stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> v.table_inner, <a href="big_vector.md#0x1_big_vector_empty">big_vector::empty</a>(bucket_size));
    };
    <a href="big_vector.md#0x1_big_vector_push_back">big_vector::push_back</a>(<a href="../../move-stdlib/doc/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(&<b>mut</b> v.table_inner, 0), val);
}
</code></pre>



</details>

<a name="0x1_smart_vector_pop_back"></a>

## Function `pop_back`

Pop an element from the end of vector <code>v</code>. It doesn't shrink the buckets even if they're empty.
Call <code>shrink_to_fit</code> explicity to deallocate empty buckets.
Aborts if <code>v</code> is empty.


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_pop_back">pop_back</a>&lt;T&gt;(v: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_pop_back">pop_back</a>&lt;T&gt;(v: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;): T {
    <b>assert</b>!(!<a href="smart_vector.md#0x1_smart_vector_is_empty">is_empty</a>(v), <a href="../../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="smart_vector.md#0x1_smart_vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>));
    <b>let</b> table_inner = &<b>mut</b> v.table_inner;
    <b>if</b> (<a href="../../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(table_inner) != 0) {
        <b>let</b> big_vec = <a href="../../move-stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(table_inner);
        <b>let</b> val = <a href="big_vector.md#0x1_big_vector_pop_back">big_vector::pop_back</a>(&<b>mut</b> big_vec);
        <b>if</b> (<a href="big_vector.md#0x1_big_vector_is_empty">big_vector::is_empty</a>(&big_vec)) {
            <a href="big_vector.md#0x1_big_vector_destroy_empty">big_vector::destroy_empty</a>(big_vec)
        } <b>else</b> {
            <a href="../../move-stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(table_inner, big_vec);
        };
        val
    } <b>else</b> {
        <a href="../../move-stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> v.inline_inner)
    }
}
</code></pre>



</details>

<a name="0x1_smart_vector_remove"></a>

## Function `remove`



<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_remove">remove</a>&lt;T&gt;(v: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;, i: u64): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_remove">remove</a>&lt;T&gt;(v: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;, i: u64): T {
    <b>let</b> len = <a href="smart_vector.md#0x1_smart_vector_length">length</a>(v);
    <b>assert</b>!(i &lt; len, <a href="../../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="smart_vector.md#0x1_smart_vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>));
    <b>let</b> inline_len = <a href="../../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&v.inline_inner);
    <b>if</b> (i &lt; inline_len) {
        <a href="../../move-stdlib/doc/vector.md#0x1_vector_remove">vector::remove</a>(&<b>mut</b> v.inline_inner, i)
    } <b>else</b> {
        <a href="big_vector.md#0x1_big_vector_remove">big_vector::remove</a>(<a href="../../move-stdlib/doc/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(&<b>mut</b> v.table_inner, 0), i - inline_len)
    }
}
</code></pre>



</details>

<a name="0x1_smart_vector_swap_remove"></a>

## Function `swap_remove`

Swap the <code>i</code>th element of the vector <code>v</code> with the last element and then pop the vector.
This is O(1), but does not preserve ordering of elements in the vector.
Aborts if <code>i</code> is out of bounds.


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_swap_remove">swap_remove</a>&lt;T&gt;(v: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;, i: u64): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_swap_remove">swap_remove</a>&lt;T&gt;(v: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;, i: u64): T {
    <b>let</b> len = <a href="smart_vector.md#0x1_smart_vector_length">length</a>(v);
    <b>assert</b>!(i &lt; len, <a href="../../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="smart_vector.md#0x1_smart_vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>));
    <b>let</b> inline_len = <a href="../../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&v.inline_inner);
    <b>let</b> table_inner = &<b>mut</b> v.table_inner;
    <b>let</b> inline_inner = &<b>mut</b> v.inline_inner;
    <b>if</b> (i &gt;= inline_len) {
        <b>let</b> big_vec = <a href="../../move-stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(table_inner);
        <b>let</b> val = <a href="big_vector.md#0x1_big_vector_swap_remove">big_vector::swap_remove</a>(&<b>mut</b> big_vec, i - inline_len);
        <b>if</b> (<a href="big_vector.md#0x1_big_vector_is_empty">big_vector::is_empty</a>(&big_vec)) {
            <a href="big_vector.md#0x1_big_vector_destroy_empty">big_vector::destroy_empty</a>(big_vec)
        } <b>else</b> {
            <a href="../../move-stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(table_inner, big_vec);
        };
        val
    } <b>else</b> {
        <b>let</b> val = <a href="../../move-stdlib/doc/vector.md#0x1_vector_swap_remove">vector::swap_remove</a>(inline_inner, i);
        <b>if</b> (inline_len &lt; len) {
            <b>let</b> big_vec = <a href="../../move-stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(table_inner);
            <b>let</b> last_from_big_vec = <a href="big_vector.md#0x1_big_vector_pop_back">big_vector::pop_back</a>(&<b>mut</b> big_vec);
            <b>if</b> (<a href="big_vector.md#0x1_big_vector_is_empty">big_vector::is_empty</a>(&big_vec)) {
                <a href="big_vector.md#0x1_big_vector_destroy_empty">big_vector::destroy_empty</a>(big_vec)
            } <b>else</b> {
                <a href="../../move-stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(table_inner, big_vec);
            };
            <a href="../../move-stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(inline_inner, last_from_big_vec);
            <a href="../../move-stdlib/doc/vector.md#0x1_vector_swap">vector::swap</a>(inline_inner, i, inline_len - 1);
        };
        val
    }
}
</code></pre>



</details>

<a name="0x1_smart_vector_swap"></a>

## Function `swap`

Swap the elements at the i'th and j'th indices in the vector v. Will abort if either of i or j are out of bounds
for v.


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_swap">swap</a>&lt;T: store&gt;(v: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;, i: u64, j: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_swap">swap</a>&lt;T: store&gt;(v: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;, i: u64, j: u64) {
    <b>if</b> (i &gt; j) {
        <b>return</b> <a href="smart_vector.md#0x1_smart_vector_swap">swap</a>(v, j, i)
    };
    <b>let</b> len = <a href="smart_vector.md#0x1_smart_vector_length">length</a>(v);
    <b>assert</b>!(j &lt; len, <a href="../../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="smart_vector.md#0x1_smart_vector_EINDEX_OUT_OF_BOUNDS">EINDEX_OUT_OF_BOUNDS</a>));
    <b>let</b> inline_len = <a href="../../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&v.inline_inner);
    <b>if</b> (i &gt;= inline_len) {
        <a href="big_vector.md#0x1_big_vector_swap">big_vector::swap</a>(<a href="../../move-stdlib/doc/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(&<b>mut</b> v.table_inner, 0), i - inline_len, j - inline_len);
    } <b>else</b> <b>if</b> (j &lt; inline_len) {
        <a href="../../move-stdlib/doc/vector.md#0x1_vector_swap">vector::swap</a>(&<b>mut</b> v.inline_inner, i, j);
    } <b>else</b> {
        <b>let</b> big_vec = <a href="../../move-stdlib/doc/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(&<b>mut</b> v.table_inner, 0);
        <b>let</b> small_vec = &<b>mut</b> v.inline_inner;
        <b>let</b> element_i = <a href="../../move-stdlib/doc/vector.md#0x1_vector_swap_remove">vector::swap_remove</a>(small_vec, i);
        <b>let</b> element_j = <a href="big_vector.md#0x1_big_vector_swap_remove">big_vector::swap_remove</a>(big_vec, j - inline_len);
        <a href="../../move-stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(small_vec, element_j);
        <a href="../../move-stdlib/doc/vector.md#0x1_vector_swap">vector::swap</a>(small_vec, i, inline_len - 1);
        <a href="big_vector.md#0x1_big_vector_push_back">big_vector::push_back</a>(big_vec, element_i);
        <a href="big_vector.md#0x1_big_vector_swap">big_vector::swap</a>(big_vec, i, len - inline_len - 1);
    }
}
</code></pre>



</details>

<a name="0x1_smart_vector_reverse"></a>

## Function `reverse`

Reverse the order of the elements in the vector v in-place.


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_reverse">reverse</a>&lt;T: store&gt;(v: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_reverse">reverse</a>&lt;T: store&gt;(v: &<b>mut</b> <a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;) {
    <b>let</b> i = 0;
    <b>let</b> len = <a href="smart_vector.md#0x1_smart_vector_length">length</a>(v);
    <b>let</b> half_len = len / 2;
    <b>let</b> k = 0;
    <b>while</b> (k &lt; half_len) {
        <a href="smart_vector.md#0x1_smart_vector_swap">swap</a>(v, i + k, len - 1 - k);
        k = k + 1;
    }
}
</code></pre>



</details>

<a name="0x1_smart_vector_index_of"></a>

## Function `index_of`

Return <code>(<b>true</b>, i)</code> if <code>val</code> is in the vector <code>v</code> at index <code>i</code>.
Otherwise, returns <code>(<b>false</b>, 0)</code>.


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_index_of">index_of</a>&lt;T&gt;(v: &<a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;, val: &T): (bool, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_index_of">index_of</a>&lt;T&gt;(v: &<a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;, val: &T): (bool, u64) {
    <b>let</b> i = 0;
    <b>let</b> len = <a href="smart_vector.md#0x1_smart_vector_length">length</a>(v);
    <b>while</b> (i &lt; len) {
        <b>if</b> (<a href="smart_vector.md#0x1_smart_vector_borrow">borrow</a>(v, i) == val) {
            <b>return</b> (<b>true</b>, i)
        };
        i = i + 1;
    };
    (<b>false</b>, 0)
}
</code></pre>



</details>

<a name="0x1_smart_vector_contains"></a>

## Function `contains`

Return true if <code>val</code> is in the vector <code>v</code>.


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_contains">contains</a>&lt;T&gt;(v: &<a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;, val: &T): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_contains">contains</a>&lt;T&gt;(v: &<a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;, val: &T): bool {
    <b>if</b> (<a href="smart_vector.md#0x1_smart_vector_is_empty">is_empty</a>(v)) <b>return</b> <b>false</b>;
    <b>let</b> (exist, _) = <a href="smart_vector.md#0x1_smart_vector_index_of">index_of</a>(v, val);
    exist
}
</code></pre>



</details>

<a name="0x1_smart_vector_length"></a>

## Function `length`

Return the length of the vector.


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_length">length</a>&lt;T&gt;(v: &<a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_length">length</a>&lt;T&gt;(v: &<a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;): u64 {
    <a href="../../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&v.inline_inner) + <b>if</b> (<a href="../../move-stdlib/doc/vector.md#0x1_vector_is_empty">vector::is_empty</a>(&v.table_inner)) {
        0
    } <b>else</b> {
        <a href="big_vector.md#0x1_big_vector_length">big_vector::length</a>(<a href="../../move-stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&v.table_inner, 0))
    }
}
</code></pre>



</details>

<a name="0x1_smart_vector_is_empty"></a>

## Function `is_empty`

Return <code><b>true</b></code> if the vector <code>v</code> has no elements and <code><b>false</b></code> otherwise.


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_is_empty">is_empty</a>&lt;T&gt;(v: &<a href="smart_vector.md#0x1_smart_vector_SmartVector">smart_vector::SmartVector</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="smart_vector.md#0x1_smart_vector_is_empty">is_empty</a>&lt;T&gt;(v: &<a href="smart_vector.md#0x1_smart_vector_SmartVector">SmartVector</a>&lt;T&gt;): bool {
    <a href="smart_vector.md#0x1_smart_vector_length">length</a>(v) == 0
}
</code></pre>



</details>


[move-book]: https://move-language.github.io/move/introduction.html
