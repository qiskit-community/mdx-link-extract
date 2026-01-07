import test from 'ava'

import { extractLinks, extractLinksFromFile } from '../index'

const dedent = (s: string) => s.replace('\n    ', '')

test('extractLinks: empty string', (t) => {
  t.deepEqual(extractLinks(''), [])
})

test('extractLinks: no links', (t) => {
  t.deepEqual(extractLinks('Hello there!'), [])
})

test('extractLinks: no links with gfm', (t) => {
  const markdown = dedent(`
    ---
    title: My title
    ---

    # Heading

    > blockquote, $math$, and ~~strikethrough~~
    `)
  t.deepEqual(extractLinks(markdown), [])
})

test('extractLinks: no links with jsx', (t) => {
  const markdown = dedent(`
    <Admonition value="thing">
      children
    </Admonition>
    `)
  t.deepEqual(extractLinks(markdown), [])
})

test('extractLinks: simple link', (t) => {
  t.deepEqual(extractLinks('Hello [there!](/path)'), ['/path'])
})

test('extractLinks: simple relative link', (t) => {
  t.deepEqual(extractLinks('Hello [there!](./path)'), ['./path'])
})

test('extractLinks: simple URL', (t) => {
  t.deepEqual(extractLinks('Hello [there!](https://www.ibm.com)'), ['https://www.ibm.com'])
})

test('extractLinks: image', (t) => {
  t.deepEqual(extractLinks('![alt](/path)'), ['/path'])
})

test('extractLinks: html link', (t) => {
  t.deepEqual(extractLinks('<a href="/path">Link text</a>'), ['/path'])
})

test('extractLinks: nested in jsx', (t) => {
  const markdown = dedent(`
    <Admonition value="thing">
      [link text](/path)
    </Admonition>
    `)
  t.deepEqual(extractLinks(markdown), ['/path'])
})

test('extractLinks: multiple links', (t) => {
  const result = extractLinks('[Hello](/path1) [there!](/path2)').sort()
  const expected = ['/path1', '/path2'].sort()
  t.deepEqual(result, expected)
})

test('extractLinks: duplicate links', (t) => {
  t.deepEqual(extractLinks('[Hello](/path) [there!](/path)'), ['/path'])
})

test('extractLinks: markdown link with alt text', (t) => {
  t.deepEqual(extractLinks('Hello [there!](/path "Some alt text")'), ['/path'])
})

test('extractLinks: gfm inside link', (t) => {
  t.deepEqual(extractLinks('Hello [~~there!~~](/path "Some alt text")'), ['/path'])
})

test('extractLinks: link inside table', (t) => {
  const links = extractLinks(
    dedent(`
    | a | b |
    |:--|--:|
    | thing | [a link](/path) |
    `),
  )
  t.deepEqual(links, ['/path'])
})

test('extractLinks: gfm footnotes', (t) => {
  const links = extractLinks(
    dedent(`
    Here's a footnote[^1]

    [^1]: And the [reference](/path)
    `),
  )
  t.deepEqual(links, ['/path'])
})

test('extractLinks: appropriate jsx error message', (t) => {
  const error = t.throws(() => extractLinks('<Admonition>'))
  t.is(error.name, 'Error')
  t.is(error.message, '1:13: Expected a closing tag for `<Admonition>` (1:1) (markdown-rs:end-tag-mismatch)')
})

test('extractLinksFromFile: mdx file', async (t) => {
  const links = await extractLinksFromFile('__test__/fixtures/markdown.mdx')
  t.deepEqual(links, ['/path'])
})

test('extractLinksFromFile: notebook', async (t) => {
  const links = (await extractLinksFromFile('__test__/fixtures/markdown.ipynb')).sort()
  t.deepEqual(links, ['/path', '/path2'].sort())
})

test('extractLinksFromFile: markdown file not found', async (t) => {
  const error = await t.throwsAsync(
    async () => await extractLinksFromFile('__test__/fixtures/file_that_does_not_exist.md'),
  )
  t.is(error.name, 'Error')
  t.is(
    error.message,
    'Could not read "__test__/fixtures/file_that_does_not_exist.md": No such file or directory (os error 2)',
  )
})

test('extractLinksFromFile: invalid notebook (not JSON)', async (t) => {
  const error = await t.throwsAsync(
    async () => await extractLinksFromFile('__test__/fixtures/invalid-notebook-json.ipynb'),
  )
  t.is(error.name, 'Error')
  t.is(
    error.message,
    'Could not read "__test__/fixtures/invalid-notebook-json.ipynb": trailing comma at line 7 column 7',
  )
})

test('extractLinksFromFile: invalid notebook (bad schema)', async (t) => {
  const error = await t.throwsAsync(
    async () => await extractLinksFromFile('__test__/fixtures/invalid-notebook-schema.ipynb'),
  )
  t.is(error.name, 'Error')
  t.is(
    error.message,
    'Could not read "__test__/fixtures/invalid-notebook-schema.ipynb": missing field `source` at line 10 column 5',
  )
})
