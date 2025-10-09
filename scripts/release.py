#!/usr/bin/env python3
"""
Release Automation Script for Adaptive Pipeline

This script automates the release process including:
- Version updates across the codebase
- Git commits and tagging
- Multi-platform builds
- Binary compression
- GitHub release publication

Usage:
    python3 scripts/release.py 2.0.0 /path/to/repo --all
    python3 scripts/release.py 2.0.0 /path/to/repo --prep
    python3 scripts/release.py 2.0.0 /path/to/repo --build --publish
    python3 scripts/release.py 2.0.0 /path/to/repo --all --dry-run
"""

import argparse
import subprocess
import sys
from pathlib import Path
from typing import List, Tuple


class ReleaseAutomation:
    """Handles the complete release automation workflow"""

    # Cross-compilation platforms (fixed set)
    PLATFORMS = [
        "aarch64-apple-darwin",
        "aarch64-unknown-linux-gnu",
        "x86_64-apple-darwin",
        "x86_64-pc-windows-gnu",
        "x86_64-unknown-linux-gnu",
    ]

    # Platform-specific binary extensions
    PLATFORM_EXTENSIONS = {
        "x86_64-pc-windows-gnu": ".exe",
    }

    def __init__(self, version: str, repo_path: str, dry_run: bool = False):
        """
        Initialize release automation

        Args:
            version: Release version (e.g., "2.0.0")
            repo_path: Path to repository root
            dry_run: If True, print commands without executing
        """
        self.version = version
        self.repo_path = Path(repo_path).resolve()
        self.dry_run = dry_run

        # Validate inputs
        self._validate_version()
        self._validate_repo_path()

    def _validate_version(self):
        """Validate version format (semantic versioning)"""
        parts = self.version.split(".")
        if len(parts) != 3 or not all(p.isdigit() for p in parts):
            print(f"❌ Error: Version must be in format X.Y.Z (e.g., 2.0.0), got: {self.version}")
            sys.exit(1)

    def _validate_repo_path(self):
        """Validate repository path exists"""
        if not self.repo_path.exists():
            print(f"❌ Error: Repository path does not exist: {self.repo_path}")
            sys.exit(1)
        if not (self.repo_path / ".git").exists():
            print(f"❌ Error: Not a git repository: {self.repo_path}")
            sys.exit(1)

    def _run_command(self, cmd: str, description: str, cwd: Path = None) -> Tuple[bool, str]:
        """
        Run a shell command with error handling

        Args:
            cmd: Command to execute
            description: Human-readable description
            cwd: Working directory (defaults to repo_path)

        Returns:
            Tuple of (success, output)
        """
        if cwd is None:
            cwd = self.repo_path

        if self.dry_run:
            print(f"[DRY RUN] {description}")
            print(f"          Command: {cmd}")
            print(f"          Working dir: {cwd}")
            return True, ""

        print(f"⏳ {description}...")
        try:
            result = subprocess.run(
                cmd,
                shell=True,
                cwd=cwd,
                capture_output=True,
                text=True,
                check=False,
            )

            if result.returncode != 0:
                print(f"❌ Failed: {description}")
                print(f"   Command: {cmd}")
                print(f"   Exit code: {result.returncode}")
                if result.stderr:
                    print(f"   Error output:\n{result.stderr}")
                return False, result.stderr

            print(f"✅ {description}")
            return True, result.stdout

        except Exception as e:
            print(f"❌ Exception during: {description}")
            print(f"   Error: {e}")
            return False, str(e)

    def _get_binary_path(self, platform: str) -> Path:
        """Get the path to a platform-specific binary"""
        extension = self.PLATFORM_EXTENSIONS.get(platform, "")
        binary_name = f"adaptive_pipeline-v{self.version}-{self._platform_to_name(platform)}{extension}"
        return self.repo_path / "target" / platform / "release" / binary_name

    def _get_zip_path(self, platform: str) -> Path:
        """Get the path to a platform-specific zip file"""
        zip_name = f"adaptive_pipeline-v{self.version}-{self._platform_to_name(platform)}.zip"
        return self.repo_path / "target" / platform / "release" / zip_name

    def _platform_to_name(self, platform: str) -> str:
        """Convert platform triple to human-readable name"""
        mappings = {
            "aarch64-apple-darwin": "macos-aarch64",
            "aarch64-unknown-linux-gnu": "linux-aarch64",
            "x86_64-apple-darwin": "macos-x86_64",
            "x86_64-pc-windows-gnu": "windows-x86_64",
            "x86_64-unknown-linux-gnu": "linux-x86_64",
        }
        return mappings.get(platform, platform)

    def step_1_prepare_environment(self) -> bool:
        """Step 1: Prepare build environment"""
        print("\n" + "=" * 70)
        print("STEP 1: Prepare Environment")
        print("=" * 70)

        commands = [
            ("cargo install cross --locked --git https://github.com/cross-rs/cross --force",
             "Install/update cross compiler"),
            ("cross --version", "Verify cross installation"),
            ("unset CROSS_NO_DOCKER || true", "Unset CROSS_NO_DOCKER"),
            ("export CROSS_CONTAINER_ENGINE=docker", "Set container engine to Docker"),
        ]

        for cmd, desc in commands:
            success, _ = self._run_command(cmd, desc)
            if not success:
                print(f"❌ Environment preparation failed at: {desc}")
                return False

        return True

    def step_2_set_version(self) -> bool:
        """Step 2: Set version throughout codebase"""
        print("\n" + "=" * 70)
        print(f"STEP 2: Set Version to v{self.version}")
        print("=" * 70)

        # Get current date
        from datetime import datetime
        date_str = datetime.now().strftime("%B %d, %Y")

        # Update Cargo.toml files
        print("Updating Cargo.toml files...")

        # Update main Cargo.toml - package version
        cmd = f"sed -i '' 's/^version = \".*\"/version = \"{self.version}\"/' adaptive_pipeline/Cargo.toml"
        success, _ = self._run_command(cmd, "Update adaptive_pipeline/Cargo.toml package version")
        if not success:
            return False

        # Update main Cargo.toml - domain dependency version
        cmd = f"sed -i '' 's/adaptive-pipeline-domain = .* path = \"..\/adaptive_pipeline_domain\", version = \".*\" .*/adaptive-pipeline-domain = {{ path = \"..\/adaptive_pipeline_domain\", version = \"{self.version}\" }}/' adaptive_pipeline/Cargo.toml"
        success, _ = self._run_command(cmd, "Update adaptive_pipeline/Cargo.toml domain dependency")
        if not success:
            return False

        # Update main Cargo.toml - bootstrap dependency version
        cmd = f"sed -i '' 's/adaptive-pipeline-bootstrap = .* path = \"..\/adaptive_pipeline_bootstrap\", version = \".*\" .*/adaptive-pipeline-bootstrap = {{ path = \"..\/adaptive_pipeline_bootstrap\", version = \"{self.version}\" }}/' adaptive_pipeline/Cargo.toml"
        success, _ = self._run_command(cmd, "Update adaptive_pipeline/Cargo.toml bootstrap dependency")
        if not success:
            return False

        # Update domain Cargo.toml
        cmd = f"sed -i '' 's/^version = \".*\"/version = \"{self.version}\"/' adaptive_pipeline_domain/Cargo.toml"
        success, _ = self._run_command(cmd, "Update adaptive_pipeline_domain/Cargo.toml")
        if not success:
            return False

        # Update bootstrap Cargo.toml
        cmd = f"sed -i '' 's/^version = \".*\"/version = \"{self.version}\"/' adaptive_pipeline_bootstrap/Cargo.toml"
        success, _ = self._run_command(cmd, "Update adaptive_pipeline_bootstrap/Cargo.toml")
        if not success:
            return False

        # Update documentation files
        print("Updating documentation files...")

        doc_updates = [
            ("docs/src/introduction.md", [
                (r'^\*\*Version:\*\* .*', f'**Version:** {self.version}'),
                (r'^\*\*Date:\*\* .*', f'**Date:** {date_str}'),
            ]),
            ("adaptive_pipeline/docs/src/introduction.md", [
                (r'^\*\*Version:\*\* .*', f'**Version:** {self.version}'),
                (r'^\*\*Date:\*\* .*', f'**Date:** {date_str}'),
            ]),
        ]

        for file_path, replacements in doc_updates:
            for pattern, replacement in replacements:
                cmd = f"sed -i '' 's/{pattern}/{replacement}/' {file_path}"
                success, _ = self._run_command(cmd, f"Update {file_path}")
                if not success:
                    return False

        # Update all documentation markdown files with version headers
        cmd = f"find adaptive_pipeline/docs/src -name '*.md' -type f -exec sed -i '' 's/^\\*\\*Version:\\*\\* [0-9]\\+\\.[0-9]\\+\\.[0-9]\\+/**Version:** {self.version}/' {{}} \\;"
        success, _ = self._run_command(cmd, "Update all adaptive_pipeline/docs/src/**/*.md files")
        if not success:
            return False

        # Update roadmap
        cmd = f"sed -i '' 's/^\\*\\*Version\\*\\*: [0-9]\\+\\.[0-9]\\+\\.[0-9]\\+/**Version**: {self.version}/' docs/roadmap.md"
        success, _ = self._run_command(cmd, "Update docs/roadmap.md")
        if not success:
            return False

        print(f"✅ Version updated to v{self.version} ({date_str})")
        return True

    def step_3_commit_changes(self, message: str) -> bool:
        """Step 3: Commit version changes"""
        print("\n" + "=" * 70)
        print("STEP 3: Commit Version Changes")
        print("=" * 70)

        # Check if there are changes to commit
        success, output = self._run_command(
            "git status --porcelain",
            "Check for uncommitted changes"
        )

        if not success:
            return False

        if not output.strip():
            print("ℹ️  No changes to commit, skipping")
            return True

        success, _ = self._run_command(
            f'git add . && git commit -m "{message}" && git push',
            "Commit and push version changes"
        )
        return success

    def step_4_update_changelog(self) -> bool:
        """Step 4: Update CHANGELOG.md with git-cliff"""
        print("\n" + "=" * 70)
        print("STEP 4: Update CHANGELOG.md")
        print("=" * 70)

        # TODO: Temporarily disabled for v2.0.0 release - using manual CHANGELOG
        print("ℹ️  git-cliff step skipped (using manual CHANGELOG)")
        return True

        # success, _ = self._run_command(
        #     f"git cliff --tag v{self.version} --prepend CHANGELOG.md --unreleased",
        #     "Generate changelog with git-cliff"
        # )
        # return success

    def step_5_commit_changelog(self) -> bool:
        """Step 5: Commit CHANGELOG.md"""
        print("\n" + "=" * 70)
        print("STEP 5: Commit CHANGELOG.md")
        print("=" * 70)

        # Check if CHANGELOG.md has changes
        success, output = self._run_command(
            "git status --porcelain CHANGELOG.md",
            "Check for CHANGELOG.md changes"
        )

        if not success:
            return False

        if not output.strip():
            print("ℹ️  No changes to CHANGELOG.md, skipping")
            return True

        success, _ = self._run_command(
            'git add CHANGELOG.md && git commit -m "release: Update CHANGELOG.md" && git push',
            "Commit and push CHANGELOG.md"
        )
        return success

    def step_6_create_tag(self) -> bool:
        """Step 6: Create and push git tag"""
        print("\n" + "=" * 70)
        print(f"STEP 6: Create Git Tag v{self.version}")
        print("=" * 70)

        success, _ = self._run_command(
            f'git tag -a v{self.version} -m "Release v{self.version}" && git push origin v{self.version}',
            f"Create and push tag v{self.version}"
        )
        return success

    def step_7_build_multiplatform(self) -> bool:
        """Step 7: Build multi-platform binaries"""
        print("\n" + "=" * 70)
        print("STEP 7: Build Multi-Platform Binaries")
        print("=" * 70)

        success, _ = self._run_command(
            "make build-all-platforms",
            "Build all platform binaries"
        )
        return success

    def step_8_compress_binaries(self) -> bool:
        """Step 8: Compress binaries into zip files"""
        print("\n" + "=" * 70)
        print("STEP 8: Compress Binaries")
        print("=" * 70)

        # Create zip files for each platform
        for platform in self.PLATFORMS:
            binary_path = self._get_binary_path(platform)
            zip_path = self._get_zip_path(platform)

            # Get binary extension
            extension = self.PLATFORM_EXTENSIONS.get(platform, "")
            source_binary = self.repo_path / "target" / platform / "release" / f"adaptive_pipeline{extension}"

            # Create zip command
            zip_dir = zip_path.parent
            zip_name = zip_path.name
            binary_name = binary_path.name

            cmd = f"cd {source_binary.parent} && cp adaptive_pipeline{extension} {binary_name} && zip {zip_name} {binary_name} && rm {binary_name}"

            success, _ = self._run_command(
                cmd,
                f"Create {zip_name}"
            )

            if not success:
                print(f"❌ Failed to create zip for {platform}")
                return False

        if not self.dry_run:
            # Verify all zips were created
            missing_zips = []
            for platform in self.PLATFORMS:
                zip_path = self._get_zip_path(platform)
                if not zip_path.exists():
                    missing_zips.append(str(zip_path))

            if missing_zips:
                print(f"❌ Missing zip files:")
                for zip_file in missing_zips:
                    print(f"   - {zip_file}")
                return False

            print(f"✅ All {len(self.PLATFORMS)} zip files created successfully")

        return True

    def step_9_publish_github(self) -> bool:
        """Step 9: Publish release to GitHub"""
        print("\n" + "=" * 70)
        print("STEP 9: Publish to GitHub")
        print("=" * 70)

        # Build list of zip files
        zip_files = [str(self._get_zip_path(platform)) for platform in self.PLATFORMS]
        zip_args = " ".join(zip_files)

        success, _ = self._run_command(
            f'gh release create v{self.version} --notes-file CHANGELOG.md --title "Release v{self.version}" --latest {zip_args}',
            f"Publish GitHub release v{self.version}"
        )
        return success

    def prep_release(self) -> bool:
        """Run preparation steps (1-6)"""
        print("\n" + "🎯" * 35)
        print("PHASE: PREP RELEASE (Steps 1-6)")
        print("🎯" * 35)

        steps = [
            (self.step_1_prepare_environment, "Prepare environment"),
            (self.step_2_set_version, "Set version"),
            (lambda: self.step_3_commit_changes(f"chore: bump version to v{self.version}"), "Commit changes"),
            (self.step_4_update_changelog, "Update CHANGELOG"),
            (self.step_5_commit_changelog, "Commit CHANGELOG"),
            (self.step_6_create_tag, "Create git tag"),
        ]

        for i, (step_func, step_name) in enumerate(steps, 1):
            print(f"\n>>> Starting Step {i}/{len(steps)}: {step_name}")
            if not step_func():
                print(f"\n❌ PREP FAILED at: {step_name}")
                return False

        print("\n" + "✅" * 35)
        print("PREP RELEASE COMPLETED SUCCESSFULLY")
        print("✅" * 35)
        return True

    def build_release(self) -> bool:
        """Run build steps (7-8)"""
        print("\n" + "🔨" * 35)
        print("PHASE: BUILD RELEASE (Steps 7-8)")
        print("🔨" * 35)

        steps = [
            (self.step_7_build_multiplatform, "Build binaries"),
            (self.step_8_compress_binaries, "Compress binaries"),
        ]

        for i, (step_func, step_name) in enumerate(steps, 1):
            print(f"\n>>> Starting Step {i}/{len(steps)}: {step_name}")
            if not step_func():
                print(f"\n❌ BUILD FAILED at: {step_name}")
                return False

        print("\n" + "✅" * 35)
        print("BUILD RELEASE COMPLETED SUCCESSFULLY")
        print("✅" * 35)
        return True

    def publish_release(self) -> bool:
        """Run publish step (9)"""
        print("\n" + "📦" * 35)
        print("PHASE: PUBLISH RELEASE (Step 9)")
        print("📦" * 35)

        if not self.step_9_publish_github():
            print("\n❌ PUBLISH FAILED")
            return False

        print("\n" + "✅" * 35)
        print("PUBLISH RELEASE COMPLETED SUCCESSFULLY")
        print("✅" * 35)
        return True

    def run_all(self) -> bool:
        """Run all steps (1-9)"""
        print("\n" + "🚀" * 35)
        print(f"FULL RELEASE AUTOMATION: v{self.version}")
        print("🚀" * 35)

        if not self.prep_release():
            return False
        if not self.build_release():
            return False
        if not self.publish_release():
            return False

        print("\n" + "🎉" * 35)
        print(f"RELEASE v{self.version} COMPLETED SUCCESSFULLY!")
        print("🎉" * 35)
        print("\nNext steps:")
        print("  • Ready for post-release activities (crates.io)")
        print(f"  • GitHub release: https://github.com/abitofhelp/optimized_adaptive_pipeline_rs/releases/tag/v{self.version}")
        return True


def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(
        description="Automate the Adaptive Pipeline release process",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Full release (all steps)
  python3 scripts/release.py 2.0.0 /path/to/repo --all

  # Prep only (steps 0-5)
  python3 scripts/release.py 2.0.0 /path/to/repo --prep

  # Build only (steps 6-7)
  python3 scripts/release.py 2.0.0 /path/to/repo --build

  # Publish only (step 8)
  python3 scripts/release.py 2.0.0 /path/to/repo --publish

  # Dry run (show commands without executing)
  python3 scripts/release.py 2.0.0 /path/to/repo --all --dry-run
        """
    )

    parser.add_argument(
        "version",
        help="Release version in semantic versioning format (e.g., 2.0.0)"
    )
    parser.add_argument(
        "repopath",
        help="Path to repository root directory"
    )
    parser.add_argument(
        "--prep",
        action="store_true",
        help="Run preparation steps (1-6): version, commits, tagging"
    )
    parser.add_argument(
        "--build",
        action="store_true",
        help="Run build steps (7-8): multi-platform builds and compression"
    )
    parser.add_argument(
        "--publish",
        action="store_true",
        help="Run publish step (9): GitHub release"
    )
    parser.add_argument(
        "--all",
        action="store_true",
        help="Run all steps (1-9): complete release automation"
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print commands without executing them"
    )

    args = parser.parse_args()

    # Validate that at least one action is specified
    if not any([args.prep, args.build, args.publish, args.all]):
        parser.error("Must specify at least one action: --prep, --build, --publish, or --all")

    # Create automation instance
    automation = ReleaseAutomation(
        version=args.version,
        repo_path=args.repopath,
        dry_run=args.dry_run
    )

    # Run requested steps
    success = True

    if args.all:
        success = automation.run_all()
    else:
        if args.prep:
            success = automation.prep_release() and success
        if args.build:
            success = automation.build_release() and success
        if args.publish:
            success = automation.publish_release() and success

    # Exit with appropriate code
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
