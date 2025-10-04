// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Fundamentals</li><li class="chapter-item expanded "><a href="fundamentals/what-is-a-pipeline.html"><strong aria-hidden="true">1.</strong> What is a Pipeline?</a></li><li class="chapter-item expanded "><a href="fundamentals/core-concepts.html"><strong aria-hidden="true">2.</strong> Core Concepts</a></li><li class="chapter-item expanded "><a href="fundamentals/stages.html"><strong aria-hidden="true">3.</strong> Pipeline Stages</a></li><li class="chapter-item expanded "><a href="fundamentals/configuration.html"><strong aria-hidden="true">4.</strong> Configuration Basics</a></li><li class="chapter-item expanded "><a href="fundamentals/first-run.html"><strong aria-hidden="true">5.</strong> Running Your First Pipeline</a></li><li class="chapter-item expanded affix "><li class="part-title">Architecture</li><li class="chapter-item expanded "><a href="architecture/overview.html"><strong aria-hidden="true">6.</strong> Overview</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="architecture/layers.html"><strong aria-hidden="true">6.1.</strong> Layered Architecture</a></li><li class="chapter-item expanded "><a href="architecture/dependencies.html"><strong aria-hidden="true">6.2.</strong> Dependency Flow</a></li></ol></li><li class="chapter-item expanded "><a href="architecture/domain-model.html"><strong aria-hidden="true">7.</strong> Domain Model</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="architecture/entities.html"><strong aria-hidden="true">7.1.</strong> Entities</a></li><li class="chapter-item expanded "><a href="architecture/value-objects.html"><strong aria-hidden="true">7.2.</strong> Value Objects</a></li><li class="chapter-item expanded "><a href="architecture/aggregates.html"><strong aria-hidden="true">7.3.</strong> Aggregates</a></li></ol></li><li class="chapter-item expanded "><a href="architecture/patterns.html"><strong aria-hidden="true">8.</strong> Design Patterns</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="architecture/repository-pattern.html"><strong aria-hidden="true">8.1.</strong> Repository Pattern</a></li><li class="chapter-item expanded "><a href="architecture/service-pattern.html"><strong aria-hidden="true">8.2.</strong> Service Pattern</a></li><li class="chapter-item expanded "><a href="architecture/adapter-pattern.html"><strong aria-hidden="true">8.3.</strong> Adapter Pattern</a></li><li class="chapter-item expanded "><a href="architecture/observer-pattern.html"><strong aria-hidden="true">8.4.</strong> Observer Pattern</a></li></ol></li><li class="chapter-item expanded "><li class="part-title">Implementation</li><li class="chapter-item expanded "><a href="implementation/stages.html"><strong aria-hidden="true">9.</strong> Stage Processing</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="implementation/compression.html"><strong aria-hidden="true">9.1.</strong> Compression</a></li><li class="chapter-item expanded "><a href="implementation/encryption.html"><strong aria-hidden="true">9.2.</strong> Encryption</a></li><li class="chapter-item expanded "><a href="implementation/integrity.html"><strong aria-hidden="true">9.3.</strong> Integrity Checking</a></li></ol></li><li class="chapter-item expanded "><a href="implementation/persistence.html"><strong aria-hidden="true">10.</strong> Data Persistence</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="implementation/repositories.html"><strong aria-hidden="true">10.1.</strong> Repository Implementation</a></li><li class="chapter-item expanded "><a href="implementation/schema.html"><strong aria-hidden="true">10.2.</strong> Schema Management</a></li></ol></li><li class="chapter-item expanded "><a href="implementation/file-io.html"><strong aria-hidden="true">11.</strong> File I/O</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="implementation/chunking.html"><strong aria-hidden="true">11.1.</strong> Chunking Strategy</a></li><li class="chapter-item expanded "><a href="implementation/binary-format.html"><strong aria-hidden="true">11.2.</strong> Binary Format</a></li></ol></li><li class="chapter-item expanded "><a href="implementation/observability.html"><strong aria-hidden="true">12.</strong> Observability</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="implementation/metrics.html"><strong aria-hidden="true">12.1.</strong> Metrics Collection</a></li><li class="chapter-item expanded "><a href="implementation/logging.html"><strong aria-hidden="true">12.2.</strong> Logging</a></li></ol></li><li class="chapter-item expanded "><li class="part-title">Advanced Topics</li><li class="chapter-item expanded "><a href="advanced/concurrency.html"><strong aria-hidden="true">13.</strong> Concurrency Model</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="advanced/thread-pooling.html"><strong aria-hidden="true">13.1.</strong> Thread Pooling</a></li><li class="chapter-item expanded "><a href="advanced/resources.html"><strong aria-hidden="true">13.2.</strong> Resource Management</a></li></ol></li><li class="chapter-item expanded "><a href="advanced/performance.html"><strong aria-hidden="true">14.</strong> Performance Optimization</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="advanced/benchmarking.html"><strong aria-hidden="true">14.1.</strong> Benchmarking</a></li><li class="chapter-item expanded "><a href="advanced/profiling.html"><strong aria-hidden="true">14.2.</strong> Profiling</a></li></ol></li><li class="chapter-item expanded "><a href="advanced/extending.html"><strong aria-hidden="true">15.</strong> Extending the Pipeline</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="advanced/custom-stages.html"><strong aria-hidden="true">15.1.</strong> Custom Stages</a></li><li class="chapter-item expanded "><a href="advanced/custom-algorithms.html"><strong aria-hidden="true">15.2.</strong> Custom Algorithms</a></li></ol></li><li class="chapter-item expanded "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Formal Documentation</li><li class="chapter-item expanded "><a href="formal/requirements.html"><strong aria-hidden="true">16.</strong> Software Requirements</a></li><li class="chapter-item expanded "><a href="formal/design.html"><strong aria-hidden="true">17.</strong> Software Design</a></li><li class="chapter-item expanded "><a href="formal/testing.html"><strong aria-hidden="true">18.</strong> Test Strategy</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">API Reference</li><li class="chapter-item expanded "><a href="api/reference.html"><strong aria-hidden="true">19.</strong> Public API</a></li><li class="chapter-item expanded "><a href="api/internal.html"><strong aria-hidden="true">20.</strong> Internal APIs</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
