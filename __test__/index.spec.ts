import test from 'ava'

import { extractLinks, extractAnchors, extractFromFile } from '../index'

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

test('extractFromFile: mdx file', async (t) => {
  const result = await extractFromFile('__test__/fixtures/markdown.mdx')
  t.deepEqual(result, [['/path'], ['#example-document']])
})

test('extractFromFile: notebook', async (t) => {
  const [links, anchors] = await extractFromFile('__test__/fixtures/markdown.ipynb')
  t.deepEqual(links.sort(), ['/path', '/path2'])
  t.deepEqual(anchors, ['#example-notebook'])
})

test('extractFromFile: markdown file not found', async (t) => {
  const error = await t.throwsAsync(async () => await extractFromFile('__test__/fixtures/file_that_does_not_exist.md'))
  t.is(error.name, 'Error')

  // The error message changes depending on OS, but both are acceptable
  const acceptableMessages = [
    'Could not read "__test__/fixtures/file_that_does_not_exist.md": No such file or directory (os error 2)',
    'Could not read "__test__/fixtures/file_that_does_not_exist.md": The system cannot find the file specified. (os error 2)',
  ]
  t.assert(acceptableMessages.includes(error.message))
})

test('extractFromFile: invalid notebook (not JSON)', async (t) => {
  const error = await t.throwsAsync(async () => await extractFromFile('__test__/fixtures/invalid-notebook-json.ipynb'))
  t.is(error.name, 'Error')
  t.is(
    error.message,
    'Could not read "__test__/fixtures/invalid-notebook-json.ipynb": trailing comma at line 7 column 7',
  )
})

test('extractFromFile: invalid notebook (bad schema)', async (t) => {
  const error = await t.throwsAsync(
    async () => await extractFromFile('__test__/fixtures/invalid-notebook-schema.ipynb'),
  )
  t.is(error.name, 'Error')
  t.is(
    error.message,
    'Could not read "__test__/fixtures/invalid-notebook-schema.ipynb": missing field `source` at line 10 column 5',
  )
})

test('extractAnchors: no anchors', (t) => {
  t.deepEqual(extractAnchors(''), [])
})

test('extractAnchors: simple heading', (t) => {
  t.deepEqual(extractAnchors('# My heading'), ['#my-heading'])
})

test('extractAnchors: duplicate headings', (t) => {
  t.deepEqual(
    extractAnchors('# My heading\n\n## My heading\n\n### My heading').sort(),
    ['#my-heading', '#my-heading-1', '#my-heading-2'].sort(),
  )
})

test('extractAnchors: bold text in headings', (t) => {
  t.deepEqual(extractAnchors('# My **heading**'), ['#my-heading'])
})

test('extractAnchors: code in headings', (t) => {
  t.deepEqual(extractAnchors('# My `heading`'), ['#my-heading'])
})

test('extractAnchors: math in headings', (t) => {
  t.deepEqual(extractAnchors('## Gates $\\rightarrow$ quantum gates'), ['#gates-rightarrow-quantum-gates'])
  t.deepEqual(
    extractAnchors(
      '### Template circuits for calculating matrix elements of $\\tilde{S}$ and $\\tilde{H}$ via Hadamard test',
    ),
    ['#template-circuits-for-calculating-matrix-elements-of-tildes-and-tildeh-via-hadamard-test'],
  )
})

test('extractAnchors: mdx in headings', (t) => {
  t.deepEqual(extractAnchors('# My <B>heading</B>`'), ['#my-heading'])
})

test('extractAnchors: forbidden characters', (t) => {
  t.deepEqual(extractAnchors('## A heading with crazy punctuation.,;:!?`()"\\'), ['#a-heading-with-crazy-punctuation'])
})

test('extractAnchors: id tags', (t) => {
  t.deepEqual(extractAnchors('<span id="thing" />'), ['#thing'])
})

test('extractAnchors: duplicate id tags', (t) => {
  t.deepEqual(extractAnchors('<span id="thing" />\n\n<span id="thing" />'), ['#thing'])
})

test('extractAnchors: headings with links', (t) => {
  t.deepEqual(extractAnchors('# My [heading](/test1) with [multiple links](/test2)'), [
    '#my-heading-with-multiple-links',
  ])
})
